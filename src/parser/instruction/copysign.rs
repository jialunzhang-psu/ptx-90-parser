use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::copysign::{Copysign, DataType},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "f32" => Ok(DataType::F32),
            "f64" => Ok(DataType::F64),
            other => Err(unexpected_value(
                span,
                &[".f32", ".f64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Copysign {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "copysign" {
            return Err(unexpected_value(span, &["copysign"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Copysign {
            data_type,
            destination,
            a,
            b,
        })
    }
}
