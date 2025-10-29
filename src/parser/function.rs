use crate::r#type::common::CodeLinkage;
use crate::{
    lexer::PtxToken,
    parser::{
        ParseErrorKind, PtxParseError, PtxParser, PtxTokenStream, expect_directive_value,
        peek_directive, unexpected_value,
    },
    r#type::function::{
        EntryFunction, FuncFunction, FunctionAlias, FunctionBody, FunctionDim3,
        FunctionEntryDirective, FunctionHeaderDirective, FunctionKernelDirective,
        FunctionStatement,
    },
};

fn consume_newlines(stream: &mut PtxTokenStream) {
    while stream
        .consume_if(|token| matches!(token, PtxToken::Newline))
        .is_some()
    {}
}

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
    consume_newlines(stream);
    if let Some((token, _)) = stream.peek().ok() {
        match token {
            PtxToken::Semicolon => {
                stream.consume()?;
                return Ok(FunctionBody::default());
            }
            PtxToken::Directive(name) if name == "noreturn" => {
                stream.consume()?;
                consume_newlines(stream);
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
                skip_braced_block(stream)?;
                return Ok(FunctionBody::default());
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
        consume_newlines(stream);
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

        consume_newlines(stream);
        let (name, _) = stream.expect_identifier()?;
        consume_newlines(stream);
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
    fn parse(_stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Err(unexpected_value(
            0..0,
            &["function entry directive"],
            "parsing function entry directives is not supported yet".to_string(),
        ))
    }
}

impl PtxParser for FunctionStatement {
    fn parse(_stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Err(unexpected_value(
            0..0,
            &["function statement"],
            "parsing function statements is not supported yet".to_string(),
        ))
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
