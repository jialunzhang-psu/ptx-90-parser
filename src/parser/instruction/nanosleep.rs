use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{common::*, instruction::nanosleep::*},
};

impl PtxParser for crate::r#type::instruction::nanosleep::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(crate::r#type::instruction::nanosleep::DataType::U32),
            other => Err(unexpected_value(span, &[".u32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for crate::r#type::instruction::nanosleep::Nanosleep {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "nanosleep" {
            return Err(unexpected_value(span, &["nanosleep"], opcode));
        }

        let data_type = crate::r#type::instruction::nanosleep::DataType::parse(stream)?;
        let delay = Operand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Nanosleep { data_type, delay })
    }
}
