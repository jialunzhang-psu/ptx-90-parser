use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::subc::*},
};

impl PtxParser for ConditionCode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| directive_matches(token, "cc")) {
            stream.consume()?;
            Ok(ConditionCode::Cc)
        } else {
            Ok(ConditionCode::None)
        }
    }
}

impl PtxParser for crate::r#type::instruction::subc::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(crate::r#type::instruction::subc::DataType::U32),
            "s32" => Ok(crate::r#type::instruction::subc::DataType::S32),
            "u64" => Ok(crate::r#type::instruction::subc::DataType::U64),
            "s64" => Ok(crate::r#type::instruction::subc::DataType::S64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32", ".u64", ".s64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for crate::r#type::instruction::subc::Subc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "subc" {
            return Err(unexpected_value(span, &["subc"], opcode));
        }

        let condition_code = ConditionCode::parse(stream)?;
        let data_type = crate::r#type::instruction::subc::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let minuend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let subtrahend = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(crate::r#type::instruction::subc::Subc {
            condition_code,
            data_type,
            destination,
            minuend,
            subtrahend,
        })
    }
}
