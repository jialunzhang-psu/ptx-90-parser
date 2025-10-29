use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::selp::*},
};

impl PtxParser for crate::r#type::instruction::selp::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b16" => Ok(Self::B16),
            "b32" => Ok(Self::B32),
            "b64" => Ok(Self::B64),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "s16" => Ok(Self::S16),
            "s32" => Ok(Self::S32),
            "s64" => Ok(Self::S64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            other => Err(unexpected_value(
                span,
                &[
                    ".b16", ".b32", ".b64", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f32",
                    ".f64",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Selp {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "selp" {
            return Err(unexpected_value(span, &["selp"], opcode));
        }

        let data_type = <crate::r#type::instruction::selp::DataType as PtxParser>::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let true_value = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let false_value = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let predicate = PredicateRegister::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Selp {
            data_type,
            destination,
            true_value,
            false_value,
            predicate,
        })
    }
}
