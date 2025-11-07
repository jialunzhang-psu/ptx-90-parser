//! Original PTX specification:
//!
//! bmsk.mode.b32  d, a, b;
//! .mode = { .clamp, .wrap };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bmsk::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Clamp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".clamp").is_ok() {
                    return Ok(Mode::Clamp);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Wrap
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wrap").is_ok() {
                    return Ok(Mode::Wrap);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".clamp", ".wrap"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for BmskModeB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bmsk")?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BmskModeB32 {
                mode,
                b32,
                d,
                a,
                b,
            })
        }
    }


}

