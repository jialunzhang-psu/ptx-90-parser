use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::szext::*},
};

impl PtxParser for crate::r#type::instruction::szext::Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "clamp" => Ok(Self::Clamp),
            "wrap" => Ok(Self::Wrap),
            other => Err(unexpected_value(
                span,
                &[".clamp", ".wrap"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::szext::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(Self::U32),
            "s32" => Ok(Self::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::szext::Szext {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "szext" {
            return Err(unexpected_value(span, &["szext"], opcode));
        }

        let mode = Mode::parse(stream)?;
        let data_type = <crate::r#type::instruction::szext::DataType as PtxParser>::parse(stream)?;

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Self {
            mode,
            data_type,
            destination,
            a,
            b,
        })
    }
}
