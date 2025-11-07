//! Original PTX specification:
//!
//! max.atype         d, a, b;
//! max{.relu}.btype  d, a, b;
//! .atype = { .u16, .u32, .u64,
//! .u16x2, .s16, .s64 };
//! .btype = { .s16x2, .s32 };
//! 
//! max{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
//! max{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
//! max.f64                            d, a, b;
//! 
//! max{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
//! max{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
//! max{.NaN}{.xorsign.abs}.bf16           d, a, b;
//! max{.NaN}{.xorsign.abs}.bf16x2         d, a, b;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::max::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16x2").is_ok() {
                    return Ok(Atype::U16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Atype::U16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Atype::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Atype::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Atype::S16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Atype::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u16x2", ".u16", ".u32", ".u64", ".s16", ".s64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Btype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try S16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16x2").is_ok() {
                    return Ok(Btype::S16x2);
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
            let expected = &[".s16x2", ".s32"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for MaxAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let atype = Atype::parse(stream)?;
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
            Ok(MaxAtype {
                atype,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxReluBtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let btype = Btype::parse(stream)?;
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
            Ok(MaxReluBtype {
                relu,
                btype,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxFtzNanXorsignAbsF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let xorsign_abs = stream.expect_string(".xorsign.abs").is_ok();
            if !xorsign_abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
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
            Ok(MaxFtzNanXorsignAbsF32 {
                ftz,
                nan,
                xorsign_abs,
                f32,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxFtzNanAbsF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let abs = stream.expect_string(".abs").is_ok();
            if !abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
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
            Ok(MaxFtzNanAbsF32 {
                ftz,
                nan,
                abs,
                f32,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for MaxF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            stream.expect_string(".f64")?;
            let f64 = ();
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
            Ok(MaxF64 {
                f64,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxFtzNanXorsignAbsF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let xorsign_abs = stream.expect_string(".xorsign.abs").is_ok();
            if !xorsign_abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
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
            Ok(MaxFtzNanXorsignAbsF16 {
                ftz,
                nan,
                xorsign_abs,
                f16,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxFtzNanXorsignAbsF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let xorsign_abs = stream.expect_string(".xorsign.abs").is_ok();
            if !xorsign_abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
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
            Ok(MaxFtzNanXorsignAbsF16x2 {
                ftz,
                nan,
                xorsign_abs,
                f16x2,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxNanXorsignAbsBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let xorsign_abs = stream.expect_string(".xorsign.abs").is_ok();
            if !xorsign_abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
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
            Ok(MaxNanXorsignAbsBf16 {
                nan,
                xorsign_abs,
                bf16,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for MaxNanXorsignAbsBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("max")?;
            let saved_pos = stream.position();
            let nan = stream.expect_string(".NaN").is_ok();
            if !nan {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let xorsign_abs = stream.expect_string(".xorsign.abs").is_ok();
            if !xorsign_abs {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
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
            Ok(MaxNanXorsignAbsBf16x2 {
                nan,
                xorsign_abs,
                bf16x2,
                d,
                a,
                b,
            })
        }
    }


}

