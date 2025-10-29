use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::RegisterOperand,
        instruction::shf::{
            DataType as ShfDataType, Direction as ShfDirection, Mode as ShfMode, Shf,
        },
    },
};

impl PtxParser for ShfDirection {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "l" => Ok(ShfDirection::Left),
            "r" => Ok(ShfDirection::Right),
            other => Err(unexpected_value(span, &[".l", ".r"], format!(".{other}"))),
        }
    }
}

impl PtxParser for ShfMode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "clamp" => Ok(ShfMode::Clamp),
            "wrap" => Ok(ShfMode::Wrap),
            other => Err(unexpected_value(
                span,
                &[".clamp", ".wrap"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for ShfDataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;

        match directive.as_str() {
            "b32" => Ok(ShfDataType::B32),
            other => Err(unexpected_value(span, &[".b32"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Shf {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "shf" {
            return Err(unexpected_value(span, &["shf"], opcode));
        }

        let direction = ShfDirection::parse(stream)?;
        let mode = ShfMode::parse(stream)?;
        let data_type = ShfDataType::parse(stream)?;

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let a = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let b = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let c = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Shf {
            direction,
            mode,
            data_type,
            destination,
            a,
            b,
            c,
        })
    }
}
