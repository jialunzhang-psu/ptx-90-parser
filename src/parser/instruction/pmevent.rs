use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::pmevent::*},
};

impl PtxParser for Pmevent {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "pmevent")?;

        let is_mask = if stream.check(|token| matches!(token, PtxToken::Directive(_))) {
            let (modifier, span) = stream.expect_directive()?;
            if modifier != "mask" {
                return Err(unexpected_value(span, &[".mask"], format!(".{modifier}")));
            }
            true
        } else if stream
            .consume_if(|token| matches!(token, PtxToken::Dot))
            .is_some()
        {
            let (modifier, span) = stream.expect_identifier()?;
            if modifier != "mask" {
                return Err(unexpected_value(span, &["mask"], modifier));
            }
            true
        } else {
            false
        };

        let immediate = Immediate::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(if is_mask {
            Pmevent::Mask { mask: immediate }
        } else {
            Pmevent::Single { event: immediate }
        })
    }
}
