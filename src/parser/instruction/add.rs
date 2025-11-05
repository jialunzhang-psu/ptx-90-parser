//! Original PTX specification:
//!
//! add.type       d, a, b;
//! add{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .u16x2, .s16x2 };
//! -------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! add{.rnd}{.ftz}.f32x2      d, a, b;
//! add{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! add{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! add{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! add{.rnd}.bf16   d, a, b;
//! add{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! add{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::add::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Type::U16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Type::S16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Type::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Type::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16x2").is_ok() {
                    return Ok(Type::U16x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16x2").is_ok() {
                    return Ok(Type::S16x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".u16x2", ".s16x2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AddType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let type_ = Type::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddType {
                type_,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddSatS32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".s32")?;
            let s32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddSatS32 {
                sat,
                s32,
                d,
                a,
                b,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::add::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Rnd::Rn);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Rnd::Rz);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rm
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rm").is_ok() {
                    return Ok(Rnd::Rm);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rp").is_ok() {
                    return Ok(Rnd::Rp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn", ".rz", ".rm", ".rp"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AddRndFtzSatF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32")?;
            let f32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndFtzSatF32 {
                rnd,
                ftz,
                sat,
                f32,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddRndFtzF32x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32x2")?;
            let f32x2 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndFtzF32x2 {
                rnd,
                ftz,
                f32x2,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddRndF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".f64")?;
            let f64 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndF64 {
                rnd,
                f64,
                d,
                a,
                b,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::add::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Rnd::Rn);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AddRndFtzSatF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f16")?;
            let f16 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndFtzSatF16 {
                rnd,
                ftz,
                sat,
                f16,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddRndFtzSatF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndFtzSatF16x2 {
                rnd,
                ftz,
                sat,
                f16x2,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddRndBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".bf16")?;
            let bf16 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndBf16 {
                rnd,
                bf16,
                d,
                a,
                b,
            })
        }
    }


    impl PtxParser for AddRndBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let b = Operand::parse(stream)?;
            Ok(AddRndBf16x2 {
                rnd,
                bf16x2,
                d,
                a,
                b,
            })
        }
    }


}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::add::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Atype::F16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Atype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Rnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Rnd::Rn);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Rnd::Rz);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rm
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rm").is_ok() {
                    return Ok(Rnd::Rm);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rp").is_ok() {
                    return Ok(Rnd::Rp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn", ".rz", ".rm", ".rp"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AddRndSatF32Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("add")?;
            let saved_pos = stream.position();
            let rnd = match Rnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32")?;
            let f32 = ();
            let atype = Atype::parse(stream)?;
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let c = Operand::parse(stream)?;
            Ok(AddRndSatF32Atype {
                rnd,
                sat,
                f32,
                atype,
                d,
                a,
                c,
            })
        }
    }


}

