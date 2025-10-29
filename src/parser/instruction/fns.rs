use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::fns::*},
};

impl PtxParser for Mask {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Mask::B32(register))
            }
            "u32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Mask::U32(register))
            }
            "s32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Mask::S32(register))
            }
            other => Err(unexpected_value(
                span,
                &[".b32", ".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Base {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "b32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Base::B32(register))
            }
            "u32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Base::U32(register))
            }
            "s32" => {
                let register = RegisterOperand::parse(stream)?;
                Ok(Base::S32(register))
            }
            other => Err(unexpected_value(
                span,
                &[".b32", ".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Fns {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "fns" {
            return Err(unexpected_value(span, &["fns"], opcode));
        }

        let (data_type, dtype_span) = stream.expect_directive()?;
        if data_type != "b32" {
            return Err(unexpected_value(
                dtype_span,
                &[".b32"],
                format!(".{data_type}"),
            ));
        }

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let mask = Mask::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let base = Base::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let offset = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Fns {
            destination,
            mask,
            base,
            offset,
        })
    }
}
