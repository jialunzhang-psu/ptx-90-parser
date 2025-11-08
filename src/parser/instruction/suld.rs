//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype{.mode}  d, [a, b];  // unformatted
//!
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d };
//! .cop   = { .ca, .cg, .cs, .cv };               // cache operation
//! .vec   = { none, .v2, .v4 };
//! .dtype = { .b8 , .b16, .b32, .b64 };
//! .mode = { .trap, .clamp, .zero };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::suld::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Ca
            {
                let saved_pos = stream.position();
                if stream.expect_string(".ca").is_ok() {
                    return Ok(Cop::Ca);
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
            // Try Cv
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cv").is_ok() {
                    return Ok(Cop::Cv);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".ca", ".cg", ".cs", ".cv"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b16").is_ok() {
                    return Ok(Dtype::B16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Dtype::B32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Dtype::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try B8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b8").is_ok() {
                    return Ok(Dtype::B8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b16", ".b32", ".b64", ".b8"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Geom {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try A1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a1d").is_ok() {
                    return Ok(Geom::A1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try A2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".a2d").is_ok() {
                    return Ok(Geom::A2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Geom::_1d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Geom::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Geom::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".a1d", ".a2d", ".1d", ".2d", ".3d"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            // Try Trap
            {
                let saved_pos = stream.position();
                if stream.expect_string(".trap").is_ok() {
                    return Ok(Mode::Trap);
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".clamp", ".trap", ".zero"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &["none", ".v2", ".v4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SuldBGeomCopVecDtypeMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("suld")?;
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let cop = match Cop::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let vec = Vec::parse(stream)?;
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let mode = match Mode::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SuldBGeomCopVecDtypeMode {
                b,
                geom,
                cop,
                vec,
                dtype,
                mode,
                d,
                a,
            })
        }
    }
}
