//! Original PTX specification:
//!
//! shfl.sync.mode.b32  d{|p}, a, b, c, membermask;
//! .mode = { .up, .down, .bfly, .idx };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::shfl_sync::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Down
            {
                let saved_pos = stream.position();
                if stream.expect_string(".down").is_ok() {
                    return Ok(Mode::Down);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Bfly
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bfly").is_ok() {
                    return Ok(Mode::Bfly);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Idx
            {
                let saved_pos = stream.position();
                if stream.expect_string(".idx").is_ok() {
                    return Ok(Mode::Idx);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Up
            {
                let saved_pos = stream.position();
                if stream.expect_string(".up").is_ok() {
                    return Ok(Mode::Up);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".down", ".bfly", ".idx", ".up"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ShflSyncModeB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("shfl")?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            let saved_pos = stream.position();
            let p = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {
                Some(GeneralOperand::parse(stream)?)
            } else {
                stream.set_position(saved_pos);
                None
            };
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let membermask = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(ShflSyncModeB32 {
                sync,
                mode,
                b32,
                d,
                p,
                a,
                b,
                c,
                membermask,
            })
        }
    }


}

