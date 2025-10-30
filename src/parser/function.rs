use crate::parser::common::{invalid_literal, parse_register_name, parse_u64_literal};
use crate::r#type::common::{CodeLinkage, PredicateRegister};
use crate::{
    lexer::PtxToken,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, expect_directive_value,
        peek_directive, unexpected_value,
    },
    r#type::{
        function::{
            EntryFunction, ExternCallBlock, ExternCallSetup, FuncFunction, FunctionAlias,
            FunctionBody, FunctionDim3, FunctionEntryDirective, FunctionHeaderDirective,
            FunctionKernelDirective, FunctionStatement, RegisterDirective,
        },
        instruction::{Instruction, InstructionOpcode},
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

fn parse_register_directive(
    stream: &mut PtxTokenStream,
) -> Result<RegisterDirective, PtxParseError> {
    expect_directive_value(stream, "reg")?;

    let ty = if stream.check(|token| matches!(token, PtxToken::Directive(_))) {
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
        other => Err(unexpected_value(
            span,
            &[".reg", ".local", ".shared", ".param"],
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
    let predicate = if stream
        .consume_if(|token| matches!(token, PtxToken::At))
        .is_some()
    {
        if stream
            .consume_if(|token| matches!(token, PtxToken::Exclaim))
            .is_some()
        {
            let (_, span) = stream.peek()?;
            return Err(unexpected_value(
                span.clone(),
                &["predicate register"],
                "!".to_string(),
            ));
        }
        Some(PredicateRegister::parse(stream)?)
    } else {
        None
    };

    let opcode = InstructionOpcode::parse(stream)?;
    Ok(Instruction {
        predicate,
        opcode,
        comment: None,
        raw: String::new(),
    })
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
            if matches!(instruction.opcode, InstructionOpcode::Call(_)) {
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

fn parse_function_statement_internal(
    stream: &mut PtxTokenStream,
) -> Result<FunctionStatement, PtxParseError> {
    if let Some(label) = try_parse_label(stream)? {
        return Ok(FunctionStatement::Label(label));
    }

    if stream.check(|token| matches!(token, PtxToken::LBrace)) {
        let block = parse_extern_call_block(stream)?;
        return Ok(FunctionStatement::ExternCallBlock(block));
    }

    let instruction = parse_instruction_statement(stream)?;
    Ok(FunctionStatement::Instruction(instruction))
}

fn skip_loc_directive(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    expect_directive_value(stream, "loc")?;
    for _ in 0..3 {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::DecimalInteger(_) => {}
            other => {
                return Err(unexpected_value(
                    span.clone(),
                    &["decimal literal"],
                    format!("{other:?}"),
                ));
            }
        }
    }
    Ok(())
}

fn skip_semicolon_directive(stream: &mut PtxTokenStream, name: &str) -> Result<(), PtxParseError> {
    expect_directive_value(stream, name)?;
    loop {
        let (token, span) = stream.consume()?;
        if matches!(token, PtxToken::Semicolon) {
            break;
        }
        if matches!(token, PtxToken::RBrace) {
            return Err(unexpected_value(span.clone(), &[";"], "}".to_string()));
        }
    }
    Ok(())
}

fn skip_to_matching_rbrace(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    let mut depth = 1usize;
    while depth > 0 {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::LBrace => depth += 1,
            PtxToken::RBrace => {
                depth -= 1;
            }
            _ => {}
        }
        if depth == 0 {
            break;
        }
        if stream.is_at_end() {
            return Err(unexpected_value(
                span.clone(),
                &["}"],
                "unterminated function body".to_string(),
            ));
        }
    }
    Ok(())
}

fn skip_parenthesized(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    stream.expect(&PtxToken::LParen)?;
    let mut depth = 1usize;
    while depth > 0 {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::LParen => depth += 1,
            PtxToken::RParen => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        if depth == 0 {
            break;
        }
        if stream.is_at_end() {
            return Err(unexpected_value(
                span.clone(),
                &["("],
                "unterminated parenthesis".to_string(),
            ));
        }
    }
    Ok(())
}

fn skip_braced_block(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    let mut depth = 1usize;
    while depth > 0 {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::LBrace => depth += 1,
            PtxToken::RBrace => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        if depth == 0 {
            break;
        }
        if stream.is_at_end() {
            return Err(unexpected_value(
                span.clone(),
                &["{"],
                "unterminated function body".to_string(),
            ));
        }
    }
    Ok(())
}

fn parse_function_body(stream: &mut PtxTokenStream) -> Result<FunctionBody, PtxParseError> {
    if let Some((token, _)) = stream.peek().ok() {
        match token {
            PtxToken::Semicolon => {
                stream.consume()?;
                return Ok(FunctionBody::default());
            }
            PtxToken::Directive(name) if name == "noreturn" => {
                stream.consume()?;
                if stream
                    .consume_if(|token| matches!(token, PtxToken::Semicolon))
                    .is_some()
                {
                    return Ok(FunctionBody::default());
                }
                stream.expect(&PtxToken::LBrace)?;
                skip_braced_block(stream)?;
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
                            if matches!(name.as_str(), "reg" | "local" | "shared" | "param") {
                                body.entry_directives
                                    .push(parse_function_entry_directive_internal(stream)?);
                                continue;
                            } else {
                                parsing_entry_directives = false;
                            }
                        } else {
                            parsing_entry_directives = false;
                        }
                    }

                    if let Some((name, _)) = peek_directive(stream)? {
                        match name.as_str() {
                            "loc" => {
                                skip_loc_directive(stream)?;
                                continue;
                            }
                            "pragma" => {
                                skip_semicolon_directive(stream, "pragma")?;
                                continue;
                            }
                            "dwarf" => {
                                skip_semicolon_directive(stream, "dwarf")?;
                                continue;
                            }
                            _ => {}
                        }
                    }

                    let statement_position = stream.position();
                    match parse_function_statement_internal(stream) {
                        Ok(statement) => body.statements.push(statement),
                        Err(_err) => {
                            stream.set_position(statement_position);
                            skip_to_matching_rbrace(stream)?;
                            body.statements.clear();
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
        let directives = parse_header_directives(stream)?;
        expect_directive_value(stream, "entry")?;
        let (name, _) = stream.expect_identifier()?;
        skip_parenthesized(stream)?;
        let body = parse_function_body(stream)?;
        Ok(EntryFunction {
            name,
            directives,
            params: Vec::new(),
            body,
        })
    }
}

impl PtxParser for FuncFunction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let directives = parse_header_directives(stream)?;
        expect_directive_value(stream, "func")?;

        if stream.check(|token| matches!(token, PtxToken::LParen)) {
            skip_parenthesized(stream)?;
        }

        let (name, _) = stream.expect_identifier()?;
        skip_parenthesized(stream)?;
        let body = parse_function_body(stream)?;
        Ok(FuncFunction {
            name,
            directives,
            return_param: None,
            params: Vec::new(),
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
        parse_function_statement_internal(stream)
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
