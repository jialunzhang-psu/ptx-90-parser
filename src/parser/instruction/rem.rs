use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::rem::*},
};

impl PtxParser for crate::r#type::instruction::rem::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "u16" => Ok(crate::r#type::instruction::rem::DataType::U16),
            "u32" => Ok(crate::r#type::instruction::rem::DataType::U32),
            "u64" => Ok(crate::r#type::instruction::rem::DataType::U64),
            "s16" => Ok(crate::r#type::instruction::rem::DataType::S16),
            "s32" => Ok(crate::r#type::instruction::rem::DataType::S32),
            "s64" => Ok(crate::r#type::instruction::rem::DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u16", ".u32", ".u64", ".s16", ".s32", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Rem {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "rem" {
            return Err(unexpected_value(span, &["rem"], opcode));
        }

        let data_type = crate::r#type::instruction::rem::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let lhs = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let rhs = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Rem {
            data_type,
            destination,
            lhs,
            rhs,
        })
    }
}
