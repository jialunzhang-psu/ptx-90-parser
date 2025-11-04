use crate::parser::common::{invalid_literal, parse_register_name, parse_u64_literal};
use crate::r#type::common::CodeLinkage;
use crate::unlexer::PtxUnlexer;
use crate::{
    lexer::{PtxToken, tokenize},
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span, expect_directive_value,
        peek_directive, unexpected_value,
    },
    r#type::{
        function::{
            DwarfDirective, EntryFunction, ExternCallBlock, ExternCallSetup, FuncFunction,
            FunctionAlias, FunctionBody, FunctionDim3, FunctionEntryDirective,
            FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement, LocationDirective,
            PragmaDirective, RegisterDirective, StatementDirective, StatementSectionDirective,
        },
        instruction::Instruction,
        variable::VariableDirective,
    },
};

fn parse_header_directives(
    stream: &mut PtxTokenStream,
) -> Result<Vec<FunctionHeaderDirective>, PtxParseError> {
    let mut directives = Vec::new();
    loop {
        let Some((name, span)) = peek_directive(stream)? else {
            break;
        };
        match name.as_str() {
            "visible" | "extern" | "weak" => {
                let linkage = CodeLinkage::parse(stream)?;
                directives.push(FunctionHeaderDirective::Linkage(linkage));
            }
            "entry" | "func" | "alias" => break,
            other => {
                return Err(unexpected_value(
                    span,
                    &[".visible", ".extern", ".weak", ".entry", ".func", ".alias"],
                    format!(".{other}"),
                ));
            }
        }
    }
    Ok(directives)
}

fn parse_register_range(stream: &mut PtxTokenStream) -> Result<Option<u32>, PtxParseError> {
    if stream
        .consume_if(|token| matches!(token, PtxToken::LAngle))
        .is_none()
    {
        return Ok(None);
    }

    let (value, span) = parse_u64_literal(stream)?;
    if value > u32::MAX as u64 {
        return Err(invalid_literal(
            span.clone(),
            "register range exceeds u32::MAX",
        ));
    }
    stream.expect(&PtxToken::RAngle)?;
    Ok(Some(value as u32))
}

fn tokens_to_string(tokens: &[PtxToken], span: &Span) -> Result<String, PtxParseError> {
    PtxUnlexer::to_string(tokens)
        .map_err(|_| invalid_literal(span.clone(), "failed to serialize token sequence"))
}

fn parse_parameter_tokens(
    tokens: &[PtxToken],
    span: &Span,
) -> Result<VariableDirective, PtxParseError> {
    let serialized = tokens_to_string(tokens, span)?;
    let source = format!("{};", serialized);
    let tokenized = tokenize(&source)
        .map_err(|_| invalid_literal(span.clone(), "failed to tokenize function parameter"))?;
    let mut temp_stream = PtxTokenStream::new(&tokenized);
    let mut directive = VariableDirective::parse(&mut temp_stream)?;
    directive.raw = serialized;
    Ok(directive)
}

fn collect_parameter_tokens(
    stream: &mut PtxTokenStream,
) -> Result<(Vec<PtxToken>, Span), PtxParseError> {
    let (first_token, first_span) = stream.peek()?;
    if matches!(first_token, PtxToken::Comma | PtxToken::RParen) {
        return Err(unexpected_value(
            first_span.clone(),
            &["function parameter"],
            format!("{first_token:?}"),
        ));
    }

    let mut tokens = Vec::new();
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    loop {
        let (next_token, _) = stream.peek()?;
        if paren_depth == 0 && bracket_depth == 0 {
            if matches!(next_token, PtxToken::Comma | PtxToken::RParen) {
                break;
            }
        }

        let (token, _) = stream.consume()?;
        match token {
            PtxToken::LParen => paren_depth += 1,
            PtxToken::RParen => paren_depth = paren_depth.saturating_sub(1),
            PtxToken::LBracket => bracket_depth += 1,
            PtxToken::RBracket => bracket_depth = bracket_depth.saturating_sub(1),
            _ => {}
        }
        tokens.push(token.clone());
    }

    Ok((tokens, first_span.clone()))
}

fn parse_parameter(stream: &mut PtxTokenStream) -> Result<VariableDirective, PtxParseError> {
    let (tokens, span) = collect_parameter_tokens(stream)?;
    if tokens.is_empty() {
        return Err(unexpected_value(
            span.clone(),
            &["function parameter"],
            "".to_string(),
        ));
    }
    parse_parameter_tokens(&tokens, &span)
}

fn parse_parameter_list(
    stream: &mut PtxTokenStream,
) -> Result<Vec<VariableDirective>, PtxParseError> {
    stream.expect(&PtxToken::LParen)?;
    if stream
        .consume_if(|token| matches!(token, PtxToken::RParen))
        .is_some()
    {
        return Ok(Vec::new());
    }

    let mut params = Vec::new();
    loop {
        let param = parse_parameter(stream)?;
        params.push(param);
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_none()
        {
            break;
        }
    }
    stream.expect(&PtxToken::RParen)?;
    Ok(params)
}

fn parse_return_parameter(
    stream: &mut PtxTokenStream,
) -> Result<Option<VariableDirective>, PtxParseError> {
    if stream
        .consume_if(|token| matches!(token, PtxToken::LParen))
        .is_none()
    {
        return Ok(None);
    }

    if stream
        .consume_if(|token| matches!(token, PtxToken::RParen))
        .is_some()
    {
        return Ok(None);
    }

    let param = parse_parameter(stream)?;
    stream.expect(&PtxToken::RParen)?;
    Ok(Some(param))
}

fn parse_optional_noreturn(
    stream: &mut PtxTokenStream,
    directives: &mut Vec<FunctionHeaderDirective>,
)-> Result<bool, PtxParseError> {
    if let Some((token, _)) = stream.peek().ok() {
        if let PtxToken::Dot = token {
            // Check if it's a directive
            let saved_pos = stream.position();
            stream.consume()?; // consume dot
            if let Ok((name, _)) = stream.expect_identifier() {
                if name == "noreturn" {
                    if !directives
                        .iter()
                        .any(|directive| matches!(directive, FunctionHeaderDirective::NoReturn))
                    {
                        directives.push(FunctionHeaderDirective::NoReturn);
                    }
                    if stream
                        .consume_if(|token| matches!(token, PtxToken::Semicolon))
                        .is_some()
                    {
                        return Ok(true);
                    }
                } else {
                    stream.set_position(saved_pos);
                }
            } else {
                stream.set_position(saved_pos);
            }
        }
    }
    Ok(false)
}

fn parse_argument_strings(
    stream: &mut PtxTokenStream,
    base_span: &Span,
    raw_tokens: &mut Vec<PtxToken>,
) -> Result<Vec<String>, PtxParseError> {
    let mut arguments = Vec::new();
    let mut current_tokens: Vec<PtxToken> = Vec::new();
    let mut current_span = base_span.clone();

    while !stream.check(|token| matches!(token, PtxToken::Semicolon)) {
        let (token, span) = stream.consume()?;
        raw_tokens.push(token.clone());
        if matches!(token, PtxToken::Comma) {
            if !current_tokens.is_empty() {
                let text = tokens_to_string(&current_tokens, &current_span)?;
                arguments.push(text);
                current_tokens.clear();
            } else {
                arguments.push(String::new());
            }
        } else {
            if current_tokens.is_empty() {
                current_span = span.clone();
            }
            current_tokens.push(token.clone());
        }
    }

    if !current_tokens.is_empty() {
        let text = tokens_to_string(&current_tokens, &current_span)?;
        arguments.push(text);
    }

    stream.expect(&PtxToken::Semicolon)?;
    raw_tokens.push(PtxToken::Semicolon);
    Ok(arguments)
}

fn parse_statement_directive(
    name: &str,
    stream: &mut PtxTokenStream,
    span: Span,
) -> Result<StatementDirective, PtxParseError> {
    let mut raw_tokens = vec![PtxToken::Dot, PtxToken::Identifier(name.to_string())];
    match name {
        "loc" => {
            let (file_token, file_span) = stream.consume()?;
            raw_tokens.push(file_token.clone());
            let file_index = match file_token {
                PtxToken::DecimalInteger(value) => value.parse::<u32>().map_err(|_| {
                    invalid_literal(
                        file_span.clone(),
                        "expected 32-bit unsigned integer literal",
                    )
                })?,
                ref other => {
                    return Err(unexpected_value(
                        file_span.clone(),
                        &["decimal literal"],
                        format!("{other:?}"),
                    ));
                }
            };

            let (line_token, line_span) = stream.consume()?;
            raw_tokens.push(line_token.clone());
            let line = match line_token {
                PtxToken::DecimalInteger(value) => value.parse::<u32>().map_err(|_| {
                    invalid_literal(
                        line_span.clone(),
                        "expected 32-bit unsigned integer literal",
                    )
                })?,
                ref other => {
                    return Err(unexpected_value(
                        line_span.clone(),
                        &["decimal literal"],
                        format!("{other:?}"),
                    ));
                }
            };

            let (column_token, column_span) = stream.consume()?;
            raw_tokens.push(column_token.clone());
            let column = match column_token {
                PtxToken::DecimalInteger(value) => value.parse::<u32>().map_err(|_| {
                    invalid_literal(
                        column_span.clone(),
                        "expected 32-bit unsigned integer literal",
                    )
                })?,
                ref other => {
                    return Err(unexpected_value(
                        column_span.clone(),
                        &["decimal literal"],
                        format!("{other:?}"),
                    ));
                }
            };

            let options = Vec::new();
            if stream
                .consume_if(|token| matches!(token, PtxToken::Semicolon))
                .is_some()
            {
                raw_tokens.push(PtxToken::Semicolon);
            }

            let raw = tokens_to_string(&raw_tokens, &span)?;
            Ok(StatementDirective::Loc(LocationDirective {
                file_index,
                line,
                column,
                options,
                comment: None,
                raw,
            }))
        }
        "pragma" => {
            let arguments = parse_argument_strings(stream, &span, &mut raw_tokens)?;
            let raw = tokens_to_string(&raw_tokens, &span)?;
            Ok(StatementDirective::Pragma(PragmaDirective {
                arguments,
                comment: None,
                raw,
            }))
        }
        "dwarf" => {
            let (keyword, keyword_span) = stream.expect_identifier()?;
            raw_tokens.push(PtxToken::Identifier(keyword.clone()));
            let arguments = parse_argument_strings(stream, &keyword_span, &mut raw_tokens)?;
            let raw = tokens_to_string(&raw_tokens, &span)?;
            Ok(StatementDirective::Dwarf(DwarfDirective {
                keyword,
                arguments,
                comment: None,
                raw,
            }))
        }
        "section" => {
            let arguments = parse_argument_strings(stream, &span, &mut raw_tokens)?;
            let mut iter = arguments.into_iter();
            let name_str = iter
                .next()
                .ok_or_else(|| unexpected_value(span.clone(), &["section name"], "".to_string()))?;
            let raw = tokens_to_string(&raw_tokens, &span)?;
            Ok(StatementDirective::Section(StatementSectionDirective {
                name: name_str,
                arguments: iter.collect(),
                comment: None,
                raw,
            }))
        }
        other => Err(unexpected_value(
            span,
            &[".loc", ".pragma", ".dwarf", ".section"],
            format!(".{other}"),
        )),
    }
}

fn parse_register_directive(
    stream: &mut PtxTokenStream,
) -> Result<RegisterDirective, PtxParseError> {
    expect_directive_value(stream, "reg")?;

    let ty = if stream.check(|token| matches!(token, PtxToken::Dot)) {
        let (directive, _) = stream.expect_directive()?;
        Some(directive)
    } else {
        None
    };

    let (name, _) = if stream.check(|token| matches!(token, PtxToken::Register(_))) {
        parse_register_name(stream)?
    } else {
        stream.expect_identifier()?
    };

    let range = parse_register_range(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(RegisterDirective {
        name,
        ty,
        range,
        comment: None,
        raw: String::new(),
    })
}

fn parse_variable_entry(
    stream: &mut PtxTokenStream,
    kind: &str,
) -> Result<FunctionEntryDirective, PtxParseError> {
    let directive = VariableDirective::parse(stream)?;
    match kind {
        "local" => Ok(FunctionEntryDirective::Local(directive)),
        "param" => Ok(FunctionEntryDirective::Param(directive)),
        "shared" => Ok(FunctionEntryDirective::Shared(directive)),
        other => Err(unexpected_value(
            0..0,
            &[".local", ".param", ".shared"],
            format!(".{other}"),
        )),
    }
}

fn parse_function_entry_directive_internal(
    stream: &mut PtxTokenStream,
) -> Result<FunctionEntryDirective, PtxParseError> {
    let maybe_directive = peek_directive(stream)?;
    let (name, span) = if let Some(value) = maybe_directive {
        value
    } else {
        let (token, span) = stream
            .peek()
            .map(|(token, span)| (token.clone(), span.clone()))?;
        return Err(unexpected_value(
            span,
            &["function entry directive"],
            format!("{token:?}"),
        ));
    };

    match name.as_str() {
        "reg" => parse_register_directive(stream).map(FunctionEntryDirective::Reg),
        "local" => parse_variable_entry(stream, "local"),
        "shared" => parse_variable_entry(stream, "shared"),
        "param" => parse_variable_entry(stream, "param"),
        "pragma" | "loc" | "dwarf" => {
            let (directive_name, directive_span) = stream.expect_directive()?;
            let statement =
                parse_statement_directive(&directive_name, stream, directive_span.clone())?;
            match statement {
                StatementDirective::Pragma(pragma) => Ok(FunctionEntryDirective::Pragma(pragma)),
                StatementDirective::Loc(loc) => Ok(FunctionEntryDirective::Loc(loc)),
                StatementDirective::Dwarf(dwarf) => Ok(FunctionEntryDirective::Dwarf(dwarf)),
                _ => Err(unexpected_value(
                    directive_span,
                    &[".pragma", ".loc", ".dwarf"],
                    format!(".{directive_name}"),
                )),
            }
        }
        other => Err(unexpected_value(
            span,
            &[
                ".reg", ".local", ".shared", ".param", ".pragma", ".loc", ".dwarf",
            ],
            format!(".{other}"),
        )),
    }
}

fn try_parse_label(stream: &mut PtxTokenStream) -> Result<Option<String>, PtxParseError> {
    if !stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        return Ok(None);
    }

    let position = stream.position();
    let (name, _) = stream.expect_identifier()?;
    if stream
        .consume_if(|token| matches!(token, PtxToken::Colon))
        .is_some()
    {
        Ok(Some(name))
    } else {
        stream.set_position(position);
        Ok(None)
    }
}

fn parse_instruction_statement(stream: &mut PtxTokenStream) -> Result<Instruction, PtxParseError> {
    // The generated Instruction parser handles predicates, labels, etc.
    Instruction::parse(stream)
}

fn parse_extern_call_block(stream: &mut PtxTokenStream) -> Result<ExternCallBlock, PtxParseError> {
    stream.expect(&PtxToken::LBrace)?;

    let mut declarations = Vec::new();
    let mut setup = Vec::new();
    let mut call = None;
    let mut post_call = Vec::new();

    loop {
        if stream.check(|token| matches!(token, PtxToken::RBrace)) {
            let (_, span) = stream.consume()?;
            if call.is_none() {
                return Err(unexpected_value(
                    span.clone(),
                    &["call instruction"],
                    "}".to_string(),
                ));
            }
            break;
        }

        if stream.is_at_end() {
            return Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span: 0..0,
            });
        }

        if let Some((name, _)) = peek_directive(stream)? {
            match name.as_str() {
                "reg" | "local" | "shared" => {
                    declarations.push(parse_function_entry_directive_internal(stream)?);
                    continue;
                }
                "param" => {
                    let directive = VariableDirective::parse(stream)?;
                    setup.push(ExternCallSetup::Param(directive));
                    continue;
                }
                _ => {}
            }
        }

        let instruction = parse_instruction_statement(stream)?;
        if call.is_none() {
            if matches!(
                instruction,
                Instruction::CallUni(_)
                    | Instruction::CallUni1(_)
                    | Instruction::CallUni2(_)
                    | Instruction::CallUni3(_)
                    | Instruction::CallUni4(_)
                    | Instruction::CallUni5(_)
                    | Instruction::CallUni6(_)
                    | Instruction::CallUni7(_)
                    | Instruction::CallUni8(_)
            ) {
                call = Some(instruction);
            } else {
                setup.push(ExternCallSetup::Store(instruction));
            }
        } else {
            post_call.push(instruction);
        }
    }

    Ok(ExternCallBlock {
        declarations,
        setup,
        call: call.expect("call instruction validated before break"),
        post_call,
    })
}

fn parse_function_statement(
    stream: &mut PtxTokenStream,
) -> Result<FunctionStatement, PtxParseError> {
    if let Some(label) = try_parse_label(stream)? {
        return Ok(FunctionStatement::Label(label));
    }

    if let Some((name, _)) = peek_directive(stream)? {
        match name.as_str() {
            "loc" | "pragma" | "dwarf" | "section" => {
                let (directive_name, span) = stream.expect_directive()?;
                let directive = parse_statement_directive(&directive_name, stream, span.clone())?;
                return Ok(FunctionStatement::Directive(directive));
            }
            _ => {}
        }
    }

    if stream.check(|token| matches!(token, PtxToken::LBrace)) {
        let block = parse_extern_call_block(stream)?;
        return Ok(FunctionStatement::ExternCallBlock(block));
    }

    let instruction = parse_instruction_statement(stream)?;
    Ok(FunctionStatement::Instruction(instruction))
}

fn collect_body_tokens(
    stream: &mut PtxTokenStream,
) -> Result<(Vec<PtxToken>, Span), PtxParseError> {
    let mut tokens = Vec::new();
    let mut depth = 1usize;
    let mut first_span: Option<Span> = None;

    while depth > 0 {
        let (token, span) = stream.consume()?;
        if first_span.is_none() {
            first_span = Some(span.clone());
        }
        match token {
            PtxToken::LBrace => {
                depth += 1;
                tokens.push(token.clone());
            }
            PtxToken::RBrace => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                tokens.push(token.clone());
            }
            _ => tokens.push(token.clone()),
        }
    }

    Ok((tokens, first_span.unwrap_or(0..0)))
}

fn parse_function_body(stream: &mut PtxTokenStream) -> Result<FunctionBody, PtxParseError> {
    if let Some((token, _)) = stream.peek().ok() {
        match token {
            PtxToken::Semicolon => {
                stream.consume()?;
                return Ok(FunctionBody::default());
            }
            PtxToken::LBrace => {
                stream.consume()?;
                let mut body = FunctionBody::default();
                let mut parsing_entry_directives = true;
                loop {
                    if stream.check(|token| matches!(token, PtxToken::RBrace)) {
                        stream.consume()?;
                        break;
                    }

                    if stream.is_at_end() {
                        return Err(PtxParseError {
                            kind: ParseErrorKind::UnexpectedEof,
                            span: 0..0,
                        });
                    }

                    if parsing_entry_directives {
                        if let Some((name, _)) = peek_directive(stream)? {
                            match name.as_str() {
                                "reg" | "local" | "shared" | "param" | "pragma" | "loc"
                                | "dwarf" => {
                                    let directive =
                                        parse_function_entry_directive_internal(stream)?;
                                    body.entry_directives.push(directive);
                                    continue;
                                }
                                _ => parsing_entry_directives = false,
                            }
                        } else {
                            parsing_entry_directives = false;
                        }
                    }

                    let position = stream.position();
                    match parse_function_statement(stream) {
                        Ok(statement) => body.statements.push(statement),
                        Err(_err) => {
                            stream.set_position(position);
                            let (tokens, span) = collect_body_tokens(stream)?;
                            if !tokens.is_empty() {
                                let raw = tokens_to_string(&tokens, &span)?;
                                body.statements.push(FunctionStatement::Directive(
                                    StatementDirective::Pragma(PragmaDirective {
                                        arguments: Vec::new(),
                                        comment: None,
                                        raw,
                                    }),
                                ));
                            }
                            return Ok(body);
                        }
                    }
                }

                return Ok(body);
            }
            _ => {
                let span = stream.peek()?.1.clone();
                return Err(unexpected_value(
                    span,
                    &[";", ".noreturn", "{"],
                    format!("{token:?}"),
                ));
            }
        }
    }

    Err(PtxParseError {
        kind: ParseErrorKind::UnexpectedEof,
        span: 0..0,
    })
}

impl PtxParser for FunctionBody {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        parse_function_body(stream)
    }
}

impl PtxParser for EntryFunction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let mut directives = parse_header_directives(stream)?;
        expect_directive_value(stream, "entry")?;
        let (name, _) = stream.expect_identifier()?;
        let params = parse_parameter_list(stream)?;
        let body = if parse_optional_noreturn(stream, &mut directives)? {
            FunctionBody::default()
        } else {
            parse_function_body(stream)?
        };
        Ok(EntryFunction {
            name,
            directives,
            params,
            body,
        })
    }
}

impl PtxParser for FuncFunction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let mut directives = parse_header_directives(stream)?;
        expect_directive_value(stream, "func")?;

        let return_param = parse_return_parameter(stream)?;

        let (name, _) = stream.expect_identifier()?;
        let params = parse_parameter_list(stream)?;
        let body = if parse_optional_noreturn(stream, &mut directives)? {
            FunctionBody::default()
        } else {
            parse_function_body(stream)?
        };
        Ok(FuncFunction {
            name,
            directives,
            return_param,
            params,
            body,
        })
    }
}

impl PtxParser for FunctionAlias {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let _ = parse_header_directives(stream)?;
        expect_directive_value(stream, "alias")?;
        let (alias, _) = stream.expect_identifier()?;
        stream.expect(&PtxToken::Comma)?;
        let (target, _) = stream.expect_identifier()?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(FunctionAlias {
            alias,
            target,
            raw: String::new(),
        })
    }
}

impl PtxParser for FunctionKernelDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let position = stream.position();
        if let Ok(entry) = EntryFunction::parse(stream) {
            return Ok(FunctionKernelDirective::Entry(entry));
        }
        stream.set_position(position);
        if let Ok(func) = FuncFunction::parse(stream) {
            return Ok(FunctionKernelDirective::Func(func));
        }
        stream.set_position(position);
        let alias = FunctionAlias::parse(stream)?;
        Ok(FunctionKernelDirective::Alias(alias))
    }
}

impl PtxParser for FunctionEntryDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        parse_function_entry_directive_internal(stream)
    }
}

impl PtxParser for FunctionStatement {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        parse_function_statement(stream)
    }
}

impl PtxParser for FunctionDim3 {
    fn parse(_stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Err(unexpected_value(
            0..0,
            &["dimension literal"],
            "parsing function dimension directives is not supported yet".to_string(),
        ))
    }
}
