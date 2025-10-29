use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::xor::*},
};

impl PtxParser for crate::r#type::instruction::xor::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "pred" => Ok(crate::r#type::instruction::xor::DataType::Pred),
            "b16" => Ok(crate::r#type::instruction::xor::DataType::B16),
            "b32" => Ok(crate::r#type::instruction::xor::DataType::B32),
            "b64" => Ok(crate::r#type::instruction::xor::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".pred", ".b16", ".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Xor {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "xor" {
            return Err(unexpected_value(span, &["xor"], opcode));
        }

        let data_type = crate::r#type::instruction::xor::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Xor {
            data_type,
            destination,
            a,
            b,
        })
    }
}
