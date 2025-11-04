//! Original PTX specification:
//!
//! discard{.global}.level  [a], size;
//! .level = { .L2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::discard::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Level {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2").is_ok() {
                    return Ok(Level::L2);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for DiscardGlobalLevel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("discard")?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            let level = Level::parse(stream)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            Ok(DiscardGlobalLevel {
                global,
                level,
                a,
                size,
            })
        }
    }


}

