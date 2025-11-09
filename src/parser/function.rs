use crate::parser::common::{
    invalid_literal, parse_register_name, parse_u64_literal, try_parse_label,
};
use crate::r#type::common::{CodeLinkage, Instruction};
use crate::unlexer::PtxUnlexer;
use crate::{
    lexer::{PtxToken, tokenize},
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, Span, expect_directive_value,
        peek_directive, unexpected_value,
    },
    r#type::{
        function::{
            DwarfDirective, EntryFunction, FuncFunction, FunctionAlias, FunctionBody, FunctionDim3,
            FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement, LocationDirective,
            PragmaDirective, RegisterDirective, StatementDirective, StatementSectionDirective,
        },
        variable::VariableDirective,
    },
};

impl FunctionHeaderDirective {
    fn parse_list(stream: &mut PtxTokenStream) -> Result<Vec<Self>, PtxParseError> {
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
) -> Result<bool, PtxParseError> {
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

fn parse_block_statements(
    stream: &mut PtxTokenStream,
) -> Result<Vec<FunctionStatement>, PtxParseError> {
    let mut statements = Vec::new();

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

        let position = stream.position();
        match FunctionStatement::parse(stream) {
            Ok(statement) => statements.push(statement),
            Err(_err) => {
                stream.set_position(position);
                let (tokens, span) = collect_body_tokens(stream)?;
                if !tokens.is_empty() {
                    let raw = tokens_to_string(&tokens, &span)?;
                    statements.push(FunctionStatement::Directive(StatementDirective::Pragma(
                        PragmaDirective {
                            arguments: Vec::new(),
                            comment: None,
                            raw,
                        },
                    )));
                }
                return Ok(statements);
            }
        }
    }

    Ok(statements)
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

impl PtxParser for FunctionBody {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        match stream.peek() {
            Ok((PtxToken::Semicolon, _)) => {
                stream.consume()?;
                Ok(FunctionBody::default())
            }
            Ok((PtxToken::LBrace, _)) => {
                stream.consume()?;
                let mut body = FunctionBody::default();
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

                    let position = stream.position();
                    match FunctionStatement::parse(stream) {
                        Ok(statement) => body.statements.push(statement),
                        Err(_) => {
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

                Ok(body)
            }
            Ok((token, _)) => {
                let span = stream.peek()?.1.clone();
                Err(unexpected_value(
                    span,
                    &[";", ".noreturn", "{"],
                    format!("{token:?}"),
                ))
            }
            Err(_) => Err(PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span: 0..0,
            }),
        }
    }
}

impl PtxParser for EntryFunction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let mut directives = FunctionHeaderDirective::parse_list(stream)?;
        expect_directive_value(stream, "entry")?;
        let (name, _) = stream.expect_identifier()?;
        let params = parse_parameter_list(stream)?;
        let body = if parse_optional_noreturn(stream, &mut directives)? {
            FunctionBody::default()
        } else {
            FunctionBody::parse(stream)?
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
        let mut directives = FunctionHeaderDirective::parse_list(stream)?;
        expect_directive_value(stream, "func")?;

        let return_param = parse_return_parameter(stream)?;

        let (name, _) = stream.expect_identifier()?;
        let params = parse_parameter_list(stream)?;
        let body = if parse_optional_noreturn(stream, &mut directives)? {
            FunctionBody::default()
        } else {
            FunctionBody::parse(stream)?
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
        let _ = FunctionHeaderDirective::parse_list(stream)?;
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

impl PtxParser for FunctionStatement {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if let Some(label) = try_parse_label(stream)? {
            return Ok(FunctionStatement::Label(label));
        }

        if peek_directive(stream)?.is_some() {
            let directive = StatementDirective::parse(stream)?;
            return Ok(FunctionStatement::Directive(directive));
        }

        if stream.check(|token| matches!(token, PtxToken::LBrace)) {
            stream.consume()?;
            let block_statements = parse_block_statements(stream)?;
            return Ok(FunctionStatement::Block(block_statements));
        }

        let instruction = Instruction::parse(stream)?;
        Ok(FunctionStatement::Instruction(instruction))
    }
}

impl PtxParser for StatementDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = if let Some(value) = peek_directive(stream)? {
            value
        } else {
            let (token, span) = stream
                .peek()
                .map(|(token, span)| (token.clone(), span.clone()))?;
            return Err(unexpected_value(
                span,
                &["function directive"],
                format!("{token:?}"),
            ));
        };

        match name.as_str() {
            "reg" => RegisterDirective::parse(stream).map(StatementDirective::Reg),
            "local" => VariableDirective::parse(stream).map(StatementDirective::Local),
            "param" => VariableDirective::parse(stream).map(StatementDirective::Param),
            "shared" => VariableDirective::parse(stream).map(StatementDirective::Shared),
            "pragma" => {
                let (_, directive_span) = stream.expect_directive()?;
                let mut raw_tokens =
                    vec![PtxToken::Dot, PtxToken::Identifier("pragma".to_string())];
                let arguments = parse_argument_strings(stream, &directive_span, &mut raw_tokens)?;
                let raw = tokens_to_string(&raw_tokens, &directive_span)?;
                Ok(StatementDirective::Pragma(PragmaDirective {
                    arguments,
                    comment: None,
                    raw,
                }))
            }
            "loc" => {
                let (_, directive_span) = stream.expect_directive()?;
                let mut raw_tokens = vec![PtxToken::Dot, PtxToken::Identifier("loc".to_string())];
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

                let raw = tokens_to_string(&raw_tokens, &directive_span)?;
                Ok(StatementDirective::Loc(LocationDirective {
                    file_index,
                    line,
                    column,
                    options,
                    comment: None,
                    raw,
                }))
            }
            "dwarf" => {
                let (_, directive_span) = stream.expect_directive()?;
                let mut raw_tokens = vec![PtxToken::Dot, PtxToken::Identifier("dwarf".to_string())];
                let (keyword, keyword_span) = stream.expect_identifier()?;
                raw_tokens.push(PtxToken::Identifier(keyword.clone()));
                let arguments = parse_argument_strings(stream, &keyword_span, &mut raw_tokens)?;
                let raw = tokens_to_string(&raw_tokens, &directive_span)?;
                Ok(StatementDirective::Dwarf(DwarfDirective {
                    keyword,
                    arguments,
                    comment: None,
                    raw,
                }))
            }
            "section" => {
                let (_, directive_span) = stream.expect_directive()?;
                let mut raw_tokens =
                    vec![PtxToken::Dot, PtxToken::Identifier("section".to_string())];
                let arguments = parse_argument_strings(stream, &directive_span, &mut raw_tokens)?;
                let mut iter = arguments.into_iter();
                let name_str = iter.next().ok_or_else(|| {
                    unexpected_value(directive_span.clone(), &["section name"], "".to_string())
                })?;
                let raw = tokens_to_string(&raw_tokens, &directive_span)?;
                Ok(StatementDirective::Section(StatementSectionDirective {
                    name: name_str,
                    arguments: iter.collect(),
                    comment: None,
                    raw,
                }))
            }
            other => Err(unexpected_value(
                span,
                &[
                    ".reg", ".local", ".param", ".shared", ".pragma", ".loc", ".dwarf", ".section",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for RegisterDirective {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
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
