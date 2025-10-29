use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::RegisterOperand,
        instruction::not::{DataType, Not},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "pred" => Ok(DataType::Pred),
            "b16" => Ok(DataType::B16),
            "b32" => Ok(DataType::B32),
            "b64" => Ok(DataType::B64),
            other => Err(unexpected_value(
                span,
                &[".pred", ".b16", ".b32", ".b64"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Not {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "not" {
            return Err(unexpected_value(span, &["not"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Not {
            data_type,
            destination,
            source,
        })
    }
}
