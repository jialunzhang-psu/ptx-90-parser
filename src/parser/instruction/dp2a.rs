//! Original PTX specification:
//!
//! dp2a.mode.atype.btype  d, a, b, c;
//! .atype = .btype = { .u32, .s32 };
//! .mode = { .lo, .hi };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::dp2a::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Lo
            {
                let saved_pos = stream.position();
                if stream.expect_string(".lo").is_ok() {
                    return Ok(Mode::Lo);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Hi
            {
                let saved_pos = stream.position();
                if stream.expect_string(".hi").is_ok() {
                    return Ok(Mode::Hi);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".lo", ".hi"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Atype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Atype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Btype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Btype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dp2aModeAtypeBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("dp2a")?;
            let mode = Mode::parse(stream)?;
            let atype = Atype::parse(stream)?;
            let btype = Btype::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(Dp2aModeAtypeBtype {
                mode,
                atype,
                btype,
                d,
                a,
                b,
                c,
            })
        }
    }


}

