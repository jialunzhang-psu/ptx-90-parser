use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::shr::{DataType, Shr},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "b16" => Ok(DataType::B16),
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            "u16" => Ok(DataType::U16),
            "u32" => Ok(DataType::U32),
            "u64" => Ok(DataType::U64),
            "s16" => Ok(DataType::S16),
            "s32" => Ok(DataType::S32),
            "s64" => Ok(DataType::S64),
            other => Err(unexpected_value(
                span,
                &[
                    ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Shr {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "shr" {
            return Err(unexpected_value(span, &["shr"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Shr {
            data_type,
            destination,
            a,
            b,
        })
    }
}
