//! Original PTX specification:
//!
//! istypep.type   p, a;  // result is .pred
//! .type = { .texref, .samplerref, .surfref };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::istypep::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Samplerref
            {
                let saved_pos = stream.position();
                if stream.expect_string(".samplerref").is_ok() {
                    return Ok(Type::Samplerref);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Surfref
            {
                let saved_pos = stream.position();
                if stream.expect_string(".surfref").is_ok() {
                    return Ok(Type::Surfref);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Texref
            {
                let saved_pos = stream.position();
                if stream.expect_string(".texref").is_ok() {
                    return Ok(Type::Texref);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".samplerref", ".surfref", ".texref"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for IstypepType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("istypep")?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let p = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(IstypepType {
                type_,
                p,
                a,
            })
        }
    }


}

