//! Original PTX specification:
//!
//! sust.b.dim{.cop}.vec.ctype.mode [a, b], c;  // unformatted
//! sust.p.dim.vec.b32.mode       [a, b], c;  // formatted
//! sust.b.adim{.cop}.vec.ctype.mode   [a, b], c;  // unformatted
//! .cop   = { .wb, .cg, .cs, .wt };                     // cache operation
//! .vec   = { none, .v2, .v4 };
//! .ctype = { .b8 , .b16, .b32, .b64 };
//! .mode  = { .trap, .clamp, .zero };
//! .dim   = { .1d, .2d, .3d };
//! .adim  = { .a1d, .a2d };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sust::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dim {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Dim::_1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Dim::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Dim::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".1d", ".2d", ".3d"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Cop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Wb
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wb").is_ok() {
                    return Ok(Cop::Wb);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cg
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cg").is_ok() {
                    return Ok(Cop::Cg);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Cs
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cs").is_ok() {
                    return Ok(Cop::Cs);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Wt
            {
                let saved_pos = stream.position();
                if stream.expect_string(".wt").is_ok() {
                    return Ok(Cop::Wt);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".wb", ".cg", ".cs", ".wt"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Ctype::B8);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Ctype::B16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Ctype::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Ctype::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b8", ".b16", ".b32", ".b64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Trap
            {
                let saved_pos = stream.position();
                if stream.expect_string(".trap").is_ok() {
                    return Ok(Mode::Trap);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Clamp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".clamp").is_ok() {
                    return Ok(Mode::Clamp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Zero
            {
                let saved_pos = stream.position();
                if stream.expect_string(".zero").is_ok() {
                    return Ok(Mode::Zero);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".trap", ".clamp", ".zero"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Adim {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try A1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a1d").is_ok() {
                    return Ok(Adim::A1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try A2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a2d").is_ok() {
                    return Ok(Adim::A2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".a1d", ".a2d"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vec {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try None
            {
                let saved_pos = stream.position();
                if stream.expect_string("none").is_ok() {
                    return Ok(Vec::None);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try V2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v2").is_ok() {
                    return Ok(Vec::V2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try V4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v4").is_ok() {
                    return Ok(Vec::V4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &["none", ".v2", ".v4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SustBDimCopVecCtypeMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sust")?;
            stream.expect_string(".b")?;
            let b = ();
            let dim = Dim::parse(stream)?;
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let vec = Vec::parse(stream)?;
            let ctype = Ctype::parse(stream)?;
            let mode = Mode::parse(stream)?;
            stream.expect(&PtxToken::LBracket)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b2 = Operand::parse(stream)?;
            stream.expect(&PtxToken::RBracket)?;
            let a = (a, b2);
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(SustBDimCopVecCtypeMode {
                b,
                dim,
                cop,
                vec,
                ctype,
                mode,
                a,
                c,
            })
        }
    }


    impl PtxParser for SustPDimVecB32Mode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sust")?;
            stream.expect_string(".p")?;
            let p = ();
            let dim = Dim::parse(stream)?;
            let vec = Vec::parse(stream)?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let mode = Mode::parse(stream)?;
            stream.expect(&PtxToken::LBracket)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            stream.expect(&PtxToken::RBracket)?;
            let a = (a, b);
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(SustPDimVecB32Mode {
                p,
                dim,
                vec,
                b32,
                mode,
                a,
                c,
            })
        }
    }


    impl PtxParser for SustBAdimCopVecCtypeMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sust")?;
            stream.expect_string(".b")?;
            let b = ();
            let adim = Adim::parse(stream)?;
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let vec = Vec::parse(stream)?;
            let ctype = Ctype::parse(stream)?;
            let mode = Mode::parse(stream)?;
            stream.expect(&PtxToken::LBracket)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b2 = Operand::parse(stream)?;
            stream.expect(&PtxToken::RBracket)?;
            let a = (a, b2);
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(SustBAdimCopVecCtypeMode {
                b,
                adim,
                cop,
                vec,
                ctype,
                mode,
                a,
                c,
            })
        }
    }


}

