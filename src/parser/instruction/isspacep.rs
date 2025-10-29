use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::isspacep::*},
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

impl PtxParser for Isspacep {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "isspacep" {
            return Err(unexpected_value(span, &["isspacep"], opcode));
        }

        let space = Space::parse(stream)?;
        let predicate = PredicateRegister::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let address = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Isspacep {
            space,
            predicate,
            address,
        })
    }
}
