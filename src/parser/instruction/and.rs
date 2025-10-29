use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::and::{And, DataType},
    },
};

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
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

impl PtxParser for And {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "and" {
            return Err(unexpected_value(span, &["and"], opcode));
        }

        let data_type = DataType::parse(stream)?;
        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(And {
            data_type,
            destination,
            a,
            b,
        })
    }
}
