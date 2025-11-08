//! Original PTX specification:
//!
//! sured.b.op.geom.ctype.mode [a,b],c; // byte addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .u32, .u64, .s32, .b32, .s64 };  // for sured.b
//! .mode  = { .trap, .clamp, .zero };
//! ----------------------------------------------------
//! sured.p.op.geom.ctype.mode [a,b],c; // sample addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .b32, .b64 };                    // for sured.p
//! .mode  = { .trap, .clamp, .zero };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sured::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Ctype::U32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Ctype::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Ctype::S32);
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
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Ctype::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u32", ".u64", ".s32", ".b32", ".s64"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Geom {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Geom::_1d);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".1d", ".2d", ".3d"];
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

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Op::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".add", ".min", ".max", ".and", ".or"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SuredBOpGeomCtypeMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sured")?;
            stream.expect_string(".b")?;
            let b = ();
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let a = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SuredBOpGeomCtypeMode {
                b,
                op,
                geom,
                ctype,
                mode,
                a,
                c,
            })
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::sured::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Ctype::B32);
                }
                stream.set_position(saved_pos);
            }
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".b64"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Geom {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Geom::_1d);
                }
                stream.set_position(saved_pos);
            }
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
            let expected = &[".1d", ".2d", ".3d"];
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

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Op::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".add", ".min", ".max", ".and", ".or"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for SuredPOpGeomCtypeMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sured")?;
            stream.expect_string(".p")?;
            let p = ();
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let geom = Geom::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let mode = Mode::parse(stream)?;
            stream.expect_complete()?;
            let a = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SuredPOpGeomCtypeMode {
                p,
                op,
                geom,
                ctype,
                mode,
                a,
                c,
            })
        }
    }
}
