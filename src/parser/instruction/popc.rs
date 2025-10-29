use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::popc::*},
};

impl PtxParser for crate::r#type::instruction::popc::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "b32" => Ok(crate::r#type::instruction::popc::DataType::B32),
            "b64" => Ok(crate::r#type::instruction::popc::DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Popc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "popc" {
            return Err(unexpected_value(span, &["popc"], opcode));
        }

        let data_type = crate::r#type::instruction::popc::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Popc {
            data_type,
            destination,
            source,
        })
    }
}
