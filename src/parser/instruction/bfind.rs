use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::bfind::*},
};

impl PtxParser for crate::r#type::instruction::bfind::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(crate::r#type::instruction::bfind::DataType::U32),
            "u64" => Ok(crate::r#type::instruction::bfind::DataType::U64),
            "s32" => Ok(crate::r#type::instruction::bfind::DataType::S32),
            "s64" => Ok(crate::r#type::instruction::bfind::DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64", ".s32", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Bfind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "bfind" {
            return Err(unexpected_value(span, &["bfind"], opcode));
        }

        let shiftamt = stream
            .consume_if(|token| matches!(token, PtxToken::Directive(name) if name == "shiftamt"))
            .is_some();

        let data_type = crate::r#type::instruction::bfind::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(if shiftamt {
            Bfind::ShiftAmount {
                data_type,
                destination,
                source,
            }
        } else {
            Bfind::Plain {
                data_type,
                destination,
                source,
            }
        })
    }
}
