use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::sad::*},
};

impl PtxParser for crate::r#type::instruction::sad::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "u16" => Ok(crate::r#type::instruction::sad::DataType::U16),
            "u32" => Ok(crate::r#type::instruction::sad::DataType::U32),
            "u64" => Ok(crate::r#type::instruction::sad::DataType::U64),
            "s16" => Ok(crate::r#type::instruction::sad::DataType::S16),
            "s32" => Ok(crate::r#type::instruction::sad::DataType::S32),
            "s64" => Ok(crate::r#type::instruction::sad::DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u16", ".u32", ".u64", ".s16", ".s32", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Sad {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "sad" {
            return Err(unexpected_value(span, &["sad"], opcode));
        }

        let data_type = crate::r#type::instruction::sad::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Sad {
            data_type,
            destination,
            a,
            b,
            c,
        })
    }
}
