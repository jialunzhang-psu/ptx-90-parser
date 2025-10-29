use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::stacksave::*},
};

impl PtxParser for crate::r#type::instruction::stacksave::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            other => Err(unexpected_value(
                span,
                &[".u32", ".u64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Stacksave {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "stacksave" {
            return Err(unexpected_value(span, &["stacksave"], opcode));
        }

        let data_type = crate::r#type::instruction::stacksave::DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Stacksave {
            data_type,
            destination,
        })
    }
}
