use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::call::*},
};

fn consume_newlines(stream: &mut PtxTokenStream) {
    while stream
        .consume_if(|token| matches!(token, PtxToken::Newline))
        .is_some()
    {}
}

fn is_numeric_token(token: &PtxToken) -> bool {
    matches!(
        token,
        PtxToken::DecimalInteger(_)
            | PtxToken::HexInteger(_)
            | PtxToken::BinaryInteger(_)
            | PtxToken::OctalInteger(_)
            | PtxToken::FloatExponent(_)
            | PtxToken::Float(_)
            | PtxToken::HexFloat(_)
    )
}

fn parse_return_parameter(stream: &mut PtxTokenStream) -> Result<CallReturn, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
        Ok(CallReturn::Register(RegisterOperand::parse(stream)?))
    } else if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        Ok(CallReturn::Param(VariableSymbol::parse(stream)?))
    } else {
        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["register operand", "identifier"],
            format!("{token:?}"),
        ))
    }
}

fn parse_argument(stream: &mut PtxTokenStream) -> Result<CallArgument, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
        Ok(CallArgument::Register(RegisterOperand::parse(stream)?))
    } else if stream.check(is_numeric_token) {
        Ok(CallArgument::Immediate(Immediate::parse(stream)?))
    } else if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        Ok(CallArgument::Param(VariableSymbol::parse(stream)?))
    } else {
        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["register operand", "numeric literal", "identifier"],
            format!("{token:?}"),
        ))
    }
}

fn parse_argument_list(stream: &mut PtxTokenStream) -> Result<Vec<CallArgument>, PtxParseError> {
    stream.expect(&PtxToken::LParen)?;
    consume_newlines(stream);

    if stream
        .consume_if(|token| matches!(token, PtxToken::RParen))
        .is_some()
    {
        return Ok(Vec::new());
    }

    let mut arguments = Vec::new();
    loop {
        consume_newlines(stream);
        arguments.push(parse_argument(stream)?);
        consume_newlines(stream);

        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            consume_newlines(stream);
            continue;
        }

        break;
    }

    stream.expect(&PtxToken::RParen)?;
    Ok(arguments)
}

fn classify_call_target(name: String) -> CallTargetList {
    match name.chars().next() {
        Some(c) if c.is_ascii_uppercase() || c == '_' || c == '$' => {
            CallTargetList::Label(Label(name))
        }
        _ => CallTargetList::Table(VariableSymbol(name)),
    }
}

fn label_from_name(name: String) -> Label {
    Label(name)
}

fn looks_like_prototype(name: &str) -> bool {
    name.to_ascii_lowercase().contains("proto")
}

impl PtxParser for Call {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "call")?;
        let uniform = consume_directive_if(stream, "uni");
        consume_newlines(stream);

        let mut return_parameter = if stream
            .consume_if(|token| matches!(token, PtxToken::LParen))
            .is_some()
        {
            consume_newlines(stream);
            let value = parse_return_parameter(stream)?;
            consume_newlines(stream);
            stream.expect(&PtxToken::RParen)?;
            consume_newlines(stream);
            stream.expect(&PtxToken::Comma)?;
            consume_newlines(stream);
            Some(value)
        } else {
            None
        };

        consume_newlines(stream);

        let kind = if stream
            .check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace))
        {
            let pointer = RegisterOperand::parse(stream)?;
            consume_newlines(stream);
            stream.expect(&PtxToken::Comma)?;
            consume_newlines(stream);

            let requires_arguments = return_parameter.is_some();
            let (arguments, has_arguments) =
                if stream.check(|token| matches!(token, PtxToken::LParen)) {
                    let args = parse_argument_list(stream)?;
                    consume_newlines(stream);
                    stream.expect(&PtxToken::Comma)?;
                    consume_newlines(stream);
                    (args, true)
                } else {
                    if requires_arguments {
                        let (token, span) = stream.peek()?;
                        return Err(unexpected_value(span.clone(), &["("], format!("{token:?}")));
                    }
                    (Vec::new(), false)
                };

            let (target_name, _) = stream.expect_identifier()?;
            let is_prototype = looks_like_prototype(&target_name);
            let prototype = label_from_name(target_name.clone());
            let targets = classify_call_target(target_name);
            consume_newlines(stream);
            stream.expect(&PtxToken::Semicolon)?;

            let ret_param = return_parameter.take();

            if is_prototype {
                if let Some(ret) = ret_param {
                    CallKind::IndirectPrototypeReturnAndArguments {
                        return_parameter: ret,
                        pointer,
                        arguments,
                        prototype,
                    }
                } else if has_arguments {
                    CallKind::IndirectPrototypeArguments {
                        pointer,
                        arguments,
                        prototype,
                    }
                } else {
                    CallKind::IndirectPrototype { pointer, prototype }
                }
            } else if let Some(ret) = ret_param {
                CallKind::IndirectTargetsReturnAndArguments {
                    return_parameter: ret,
                    pointer,
                    arguments,
                    targets,
                }
            } else if has_arguments {
                CallKind::IndirectTargetsArguments {
                    pointer,
                    arguments,
                    targets,
                }
            } else {
                CallKind::IndirectTargets { pointer, targets }
            }
        } else {
            let callee = FunctionSymbol::parse(stream)?;
            consume_newlines(stream);

            if stream
                .consume_if(|token| matches!(token, PtxToken::Comma))
                .is_some()
            {
                consume_newlines(stream);
                let arguments = parse_argument_list(stream)?;
                consume_newlines(stream);
                stream.expect(&PtxToken::Semicolon)?;

                if let Some(ret) = return_parameter.take() {
                    CallKind::DirectReturnAndArguments {
                        return_parameter: ret,
                        callee,
                        arguments,
                    }
                } else {
                    CallKind::DirectArguments { callee, arguments }
                }
            } else {
                if return_parameter.is_some() {
                    let (token, span) = stream.peek()?;
                    return Err(unexpected_value(span.clone(), &[","], format!("{token:?}")));
                }
                stream.expect(&PtxToken::Semicolon)?;
                CallKind::Direct { callee }
            }
        };

        Ok(Call { uniform, kind })
    }
}
