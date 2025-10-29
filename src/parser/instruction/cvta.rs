use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Immediate, RegisterOperand, VariableSymbol},
        instruction::cvta::{Cvta, GenericSource, Size, Space, ToAddressSpace, ToGeneric},
    },
};

impl PtxParser for Space {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "const" => Ok(Space::Const),
            "global" => Ok(Space::Global),
            "local" => Ok(Space::Local),
            "shared" => {
                if stream.check(|token| matches!(token, PtxToken::Colon)) {
                    stream.expect_double_colon()?;
                    let (modifier, modifier_span) = stream.expect_identifier()?;
                    match modifier.as_str() {
                        "cta" => Ok(Space::SharedCta),
                        "cluster" => Ok(Space::SharedCluster),
                        other => Err(unexpected_value(modifier_span, &["cta", "cluster"], other)),
                    }
                } else {
                    Ok(Space::Shared)
                }
            }
            "param" => {
                if stream.check(|token| matches!(token, PtxToken::Colon)) {
                    stream.expect_double_colon()?;
                    let (modifier, modifier_span) = stream.expect_identifier()?;
                    match modifier.as_str() {
                        "entry" => Ok(Space::ParamEntry),
                        other => Err(unexpected_value(modifier_span, &["entry"], other)),
                    }
                } else {
                    Ok(Space::Param)
                }
            }
            other => Err(unexpected_value(
                span,
                &[
                    ".const",
                    ".global",
                    ".local",
                    ".shared",
                    ".shared::cta",
                    ".shared::cluster",
                    ".param",
                    ".param::entry",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Size {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "u32" => Ok(Size::U32),
            "u64" => Ok(Size::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for GenericSource {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| {
            matches!(token, PtxToken::Register(_)) || matches!(token, PtxToken::LBrace)
        }) {
            let register = RegisterOperand::parse(stream)?;
            return Ok(GenericSource::Register(register));
        }

        let variable = VariableSymbol::parse(stream)?;
        if stream
            .consume_if(|token| matches!(token, PtxToken::Plus))
            .is_some()
        {
            let immediate = Immediate::parse(stream)?;
            Ok(GenericSource::VariableWithImmediate {
                variable,
                immediate,
            })
        } else {
            Ok(GenericSource::Variable(variable))
        }
    }
}

impl PtxParser for ToGeneric {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let space = Space::parse(stream)?;
        let size = Size::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = GenericSource::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(ToGeneric {
            space,
            size,
            destination,
            source,
        })
    }
}

impl PtxParser for ToAddressSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let space = Space::parse(stream)?;
        let size = Size::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(ToAddressSpace {
            space,
            size,
            destination,
            source,
        })
    }
}

impl PtxParser for Cvta {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "cvta" {
            return Err(unexpected_value(span, &["cvta"], opcode));
        }

        let &(ref token, ref token_span) = stream.peek()?;
        match token {
            PtxToken::Directive(name) if name == "to" => {
                stream.consume()?;
                let instruction = ToAddressSpace::parse(stream)?;
                Ok(Cvta::ToAddressSpace(instruction))
            }
            PtxToken::Directive(_) => {
                let instruction = ToGeneric::parse(stream)?;
                Ok(Cvta::ToGeneric(instruction))
            }
            other => Err(unexpected_value(
                token_span.clone(),
                &[".to", ".const", ".global", ".local", ".shared", ".param"],
                format!("{other:?}"),
            )),
        }
    }
}
