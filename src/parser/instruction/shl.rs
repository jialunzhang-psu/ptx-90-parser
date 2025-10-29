use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::shl::{DataType as ShlDataType, Shl},
    },
};

impl PtxParser for ShlDataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "b16" => Ok(ShlDataType::B16),
            "b32" => Ok(ShlDataType::B32),
            "b64" => Ok(ShlDataType::B64),
            other => Err(unexpected_value(
                span,
                &[".b16", ".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Shl {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "shl" {
            return Err(unexpected_value(span, &["shl"], opcode));
        }

        let data_type = ShlDataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Shl {
            data_type,
            destination,
            a,
            b,
        })
    }
}
