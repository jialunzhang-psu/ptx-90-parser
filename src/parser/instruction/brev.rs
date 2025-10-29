use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::brev::*},
};

impl PtxParser for crate::r#type::instruction::brev::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "b32" => Ok(crate::r#type::instruction::brev::DataType::B32),
            "b64" => Ok(crate::r#type::instruction::brev::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Brev {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "brev" {
            return Err(unexpected_value(span, &["brev"], opcode));
        }

        let data_type = crate::r#type::instruction::brev::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Brev {
            data_type,
            destination,
            source,
        })
    }
}
