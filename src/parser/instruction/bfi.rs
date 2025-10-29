use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::RegisterOperand, instruction::bfi::*},
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Bfi {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "bfi" {
            return Err(unexpected_value(span, &["bfi"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let base = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let position = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let length = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Bfi {
            data_type,
            destination,
            source,
            base,
            position,
            length,
        })
    }
}
