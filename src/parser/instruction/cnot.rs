use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::cnot::*},
};

impl PtxParser for crate::r#type::instruction::cnot::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "b16" => Ok(crate::r#type::instruction::cnot::DataType::B16),
            "b32" => Ok(crate::r#type::instruction::cnot::DataType::B32),
            "b64" => Ok(crate::r#type::instruction::cnot::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b16", ".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Cnot {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "cnot" {
            return Err(unexpected_value(span, &["cnot"], opcode));
        }

        let data_type = crate::r#type::instruction::cnot::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Cnot {
            data_type,
            destination,
            source,
        })
    }
}
