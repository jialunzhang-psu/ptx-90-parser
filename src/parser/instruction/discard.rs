use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::{
        common::AddressOperand,
        instruction::discard::{Discard, Level, Size, Space},
    },
};

impl PtxParser for Space {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "global" => Ok(Space::Global),
            other => Err(unexpected_value(span, &[".global"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Level {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "L2" => Ok(Level::L2),
            other => Err(unexpected_value(span, &[".L2"], format!(".{other}"))),
        }
    }
}

impl PtxParser for Size {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (token, span) = stream.consume()?;
        let span = span.clone();
        match token {
            PtxToken::DecimalInteger(value) if value == "128" => Ok(Size::Bytes128),
            other => Err(unexpected_value(span, &["128"], format!("{other:?}"))),
        }
    }
}

impl PtxParser for Discard {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "discard" {
            return Err(unexpected_value(span, &["discard"], opcode));
        }

        let space = if stream
            .check(|token| matches!(token, PtxToken::Directive(name) if name == "global"))
        {
            Some(Space::parse(stream)?)
        } else {
            None
        };

        let level = Level::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let size = Size::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Discard {
            space,
            level,
            address,
            size,
        })
    }
}
