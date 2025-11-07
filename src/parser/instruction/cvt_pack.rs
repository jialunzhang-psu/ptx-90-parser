//! Original PTX specification:
//!
//! cvt.pack.sat.convertType.abType  d, a, b;
//! .convertType  = { .u16, .s16 };
//! .abType       = { .s32 };
//! ----------------------------------------------------------------
//! cvt.pack.sat.convertType.abType.cType  d, a, b, c;
//! .convertType  = { .u2, .s2, .u4, .s4, .u8, .s8 };
//! .abType       = { .s32 };
//! .cType        = { .b32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cvt_pack::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Abtype::S32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Converttype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Converttype::U16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Converttype::S16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u16", ".s16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CvtPackSatConverttypeAbtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".pack")?;
            let pack = ();
            stream.expect_complete()?;
            stream.expect_string(".sat")?;
            let sat = ();
            stream.expect_complete()?;
            let converttype = Converttype::parse(stream)?;
            stream.expect_complete()?;
            let abtype = Abtype::parse(stream)?;
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
            Ok(CvtPackSatConverttypeAbtype {
                pack,
                sat,
                converttype,
                abtype,
                d,
                a,
                b,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cvt_pack::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Abtype::S32);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Converttype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u2").is_ok() {
                    return Ok(Converttype::U2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try S2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s2").is_ok() {
                    return Ok(Converttype::S2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u4").is_ok() {
                    return Ok(Converttype::U4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s4").is_ok() {
                    return Ok(Converttype::S4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Converttype::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Converttype::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u2", ".s2", ".u4", ".s4", ".u8", ".s8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CvtPackSatConverttypeAbtypeCtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".pack")?;
            let pack = ();
            stream.expect_complete()?;
            stream.expect_string(".sat")?;
            let sat = ();
            stream.expect_complete()?;
            let converttype = Converttype::parse(stream)?;
            stream.expect_complete()?;
            let abtype = Abtype::parse(stream)?;
            stream.expect_complete()?;
            let ctype = Ctype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtPackSatConverttypeAbtypeCtype {
                pack,
                sat,
                converttype,
                abtype,
                ctype,
                d,
                a,
                b,
                c,
            })
        }
    }


}

