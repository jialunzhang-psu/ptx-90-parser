use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::prefetchu::*},
};

fn level_from_directive(directive: &str, span: Span) -> Result<PrefetchuLevel, PtxParseError> {
    match directive {
        "L1" => Ok(PrefetchuLevel::L1),
        other => Err(unexpected_value(span, &[".L1"], format!(".{other}"))),
    }
}

impl PtxParser for PrefetchuLevel {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        level_from_directive(&directive, span)
    }
}

impl PtxParser for Prefetchu {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "prefetchu")?;
        let level = PrefetchuLevel::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Prefetchu { level, address })
    }
}
