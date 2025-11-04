//! Original PTX specification:
//!
//! prmt.b32{.mode}  d, a, b, c;
//! .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::prmt::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F4e
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f4e").is_ok() {
                    return Ok(Mode::F4e);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B4e
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b4e").is_ok() {
                    return Ok(Mode::B4e);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rc8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rc8").is_ok() {
                    return Ok(Mode::Rc8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ecl
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ecl").is_ok() {
                    return Ok(Mode::Ecl);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Ecr
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ecr").is_ok() {
                    return Ok(Mode::Ecr);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rc16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rc16").is_ok() {
                    return Ok(Mode::Rc16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f4e", ".b4e", ".rc8", ".ecl", ".ecr", ".rc16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for PrmtB32Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("prmt")?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let saved_pos = stream.position();
            let mode = match Mode::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(PrmtB32Mode {
                b32,
                mode,
                d,
                a,
                b,
                c,
            })
        }
    }


}

