use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::bfe::{Bfe, DataType},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64", ".s32", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Bfe {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "bfe" {
            return Err(unexpected_value(span, &["bfe"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let bit_position = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let field_length = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Bfe {
            data_type,
            destination,
            source,
            bit_position,
            field_length,
        })
    }
}
