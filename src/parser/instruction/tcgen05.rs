use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::tcgen05::*},
};

impl PtxParser for CtaGroup {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "cta_group")?;
        stream.expect_double_colon()?;
        let (token, span) = stream.consume()?;

        match token {
            PtxToken::DecimalInteger(value) if value == "1" => Ok(CtaGroup::One),
            PtxToken::DecimalInteger(value) if value == "2" => Ok(CtaGroup::Two),
            other => Err(unexpected_value(
                span.clone(),
                &["::1", "::2"],
                match other {
                    PtxToken::DecimalInteger(value) => format!("::{value}"),
                    _ => format!("{other:?}"),
                },
            )),
        }
    }
}

impl PtxParser for StateSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "shared" => {
                stream.expect_double_colon()?;
                let (identifier, identifier_span) = stream.expect_identifier()?;
                if identifier == "cta" {
                    Ok(StateSpace::SharedCta)
                } else {
                    Err(unexpected_value(identifier_span, &["cta"], identifier))
                }
            }
            other => Err(unexpected_value(
                span,
                &[".shared::cta"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Alloc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let cta_group = CtaGroup::parse(stream)?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;

        let state_space = match peek_directive(stream)? {
            Some((directive, _)) if directive == "shared" => Some(StateSpace::parse(stream)?),
            _ => None,
        };

        expect_directive_value(stream, "b32")?;
        let destination = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let column_count = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Alloc {
            cta_group,
            state_space,
            destination,
            column_count,
        })
    }
}

impl PtxParser for Dealloc {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let cta_group = CtaGroup::parse(stream)?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;
        expect_directive_value(stream, "b32")?;
        let tensor_address = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let column_count = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Dealloc {
            cta_group,
            tensor_address,
            column_count,
        })
    }
}

impl PtxParser for RelinquishAllocPermit {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let cta_group = CtaGroup::parse(stream)?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(RelinquishAllocPermit { cta_group })
    }
}

impl PtxParser for Tcgen05 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "tcgen05")?;
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "alloc" => Ok(Tcgen05::Alloc(Alloc::parse(stream)?)),
            "dealloc" => Ok(Tcgen05::Dealloc(Dealloc::parse(stream)?)),
            "relinquish_alloc_permit" => Ok(Tcgen05::RelinquishAllocPermit(
                RelinquishAllocPermit::parse(stream)?,
            )),
            other => Err(unexpected_value(
                span,
                &[".alloc", ".dealloc", ".relinquish_alloc_permit"],
                format!(".{other}"),
            )),
        }
    }
}
