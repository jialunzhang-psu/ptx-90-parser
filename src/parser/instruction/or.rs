use crate::r#type::instruction::or;
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::or::*},
};

impl PtxParser for or::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "pred" => Ok(or::DataType::Pred),
            "b16" => Ok(or::DataType::B16),
            "b32" => Ok(or::DataType::B32),
            "b64" => Ok(or::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".pred", ".b16", ".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Or {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "or" {
            return Err(unexpected_value(span, &["or"], opcode));
        }

        let data_type = or::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Or {
            data_type,
            destination,
            a,
            b,
        })
    }
}
