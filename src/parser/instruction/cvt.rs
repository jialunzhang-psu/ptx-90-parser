//! Original PTX specification:
//!
//! cvt{.irnd}{.ftz}{.sat}.dtype.atype         d, a;  // integer rounding
//! cvt{.frnd}{.ftz}{.sat}.dtype.atype         d, a;  // fp rounding
//! cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
//! cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
//! cvt.rs{.relu}{.satfinite}.f16x2.f32        d, a, b, rbits;
//! cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
//! cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
//! cvt.rs{.relu}{.satfinite}.bf16x2.f32       d, a, b, rbits;
//! cvt.rna{.satfinite}.tf32.f32               d, a;
//! cvt.frnd2{.satfinite}{.relu}.tf32.f32      d, a;
//! cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
//! cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
//! cvt.rn{.relu}.f16x2.f8x2type              d, a;
//! cvt.rs{.relu}.satfinite.f8x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f4x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f4x2type               d, a;
//! cvt.rs{.relu}.satfinite.f4x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f6x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f6x2type               d, a;
//! cvt.rs{.relu}.satfinite.f6x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.frnd3{.satfinite}.ue8m0x2.f32          d, a, b;
//! cvt.frnd3{.satfinite}.ue8m0x2.bf16x2       d, a;
//! cvt.rn.bf16x2.ue8m0x2                      d, a;
//! .irnd   = { .rni, .rzi, .rmi, .rpi };
//! .frnd   = { .rn,  .rz,  .rm,  .rp  };
//! .frnd2  = { .rn,  .rz };
//! .frnd3  = { .rz,  .rp };
//! .dtype = .atype = { .u8,   .u16, .u32, .u64,
//! .s8,   .s16, .s32, .s64,
//! .bf16, .f16, .f32, .f64 };
//! .f8x2type = { .e4m3x2, .e5m2x2 };
//! .f4x2type = { .e2m1x2 };
//! .f6x2type = { .e2m3x2, .e3m2x2 };
//! .f4x4type = { .e2m1x4 };
//! .f8x4type = { .e4m3x4, .e5m2x4 };
//! .f6x4type = { .e2m3x4, .e3m2x4 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cvt::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Atype::Bf16);
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
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Atype::S32);
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
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Atype::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Atype::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Atype::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Atype::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Atype::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".bf16", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f16", ".f32", ".f64",
                ".u8", ".s8",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Dtype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try U16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u16").is_ok() {
                    return Ok(Dtype::U16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Dtype::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Dtype::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s16").is_ok() {
                    return Ok(Dtype::S16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Dtype::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Dtype::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Dtype::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Dtype::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Dtype::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u8").is_ok() {
                    return Ok(Dtype::U8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s8").is_ok() {
                    return Ok(Dtype::S8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".bf16", ".u16", ".u32", ".u64", ".s16", ".s32", ".s64", ".f16", ".f32", ".f64",
                ".u8", ".s8",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F4x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E2m1x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m1x2").is_ok() {
                    return Ok(F4x2type::E2m1x2);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e2m1x2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F4x4type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E2m1x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m1x4").is_ok() {
                    return Ok(F4x4type::E2m1x4);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e2m1x4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F6x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E2m3x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m3x2").is_ok() {
                    return Ok(F6x2type::E2m3x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E3m2x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e3m2x2").is_ok() {
                    return Ok(F6x2type::E3m2x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e2m3x2", ".e3m2x2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F6x4type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E2m3x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e2m3x4").is_ok() {
                    return Ok(F6x4type::E2m3x4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E3m2x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e3m2x4").is_ok() {
                    return Ok(F6x4type::E3m2x4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e2m3x4", ".e3m2x4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F8x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3x2").is_ok() {
                    return Ok(F8x2type::E4m3x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2x2").is_ok() {
                    return Ok(F8x2type::E5m2x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3x2", ".e5m2x2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for F8x4type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try E4m3x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e4m3x4").is_ok() {
                    return Ok(F8x4type::E4m3x4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try E5m2x4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".e5m2x4").is_ok() {
                    return Ok(F8x4type::E5m2x4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".e4m3x4", ".e5m2x4"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Frnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Frnd::Rn);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Frnd::Rz);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rm
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rm").is_ok() {
                    return Ok(Frnd::Rm);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rp").is_ok() {
                    return Ok(Frnd::Rp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn", ".rz", ".rm", ".rp"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Frnd2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rn
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rn").is_ok() {
                    return Ok(Frnd2::Rn);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Frnd2::Rz);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rn", ".rz"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Frnd3 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rz
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rz").is_ok() {
                    return Ok(Frnd3::Rz);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rp
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rp").is_ok() {
                    return Ok(Frnd3::Rp);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rz", ".rp"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Irnd {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Rni
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rni").is_ok() {
                    return Ok(Irnd::Rni);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Rzi
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rzi").is_ok() {
                    return Ok(Irnd::Rzi);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rmi
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rmi").is_ok() {
                    return Ok(Irnd::Rmi);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Rpi
            {
                let saved_pos = stream.position();
                if stream.expect_string(".rpi").is_ok() {
                    return Ok(Irnd::Rpi);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".rni", ".rzi", ".rmi", ".rpi"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CvtIrndFtzSatDtypeAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let saved_pos = stream.position();
            let irnd = match Irnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtIrndFtzSatDtypeAtype {
                irnd,
                ftz,
                sat,
                dtype,
                atype,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtFrndFtzSatDtypeAtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let saved_pos = stream.position();
            let frnd = match Frnd::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let dtype = Dtype::parse(stream)?;
            stream.expect_complete()?;
            let atype = Atype::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtFrndFtzSatDtypeAtype {
                frnd,
                ftz,
                sat,
                dtype,
                atype,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteF16F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd2 = Frnd2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtFrnd2ReluSatfiniteF16F32 {
                frnd2,
                relu,
                satfinite,
                f16,
                f32,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteF16x2F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd2 = Frnd2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
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
            Ok(CvtFrnd2ReluSatfiniteF16x2F32 {
                frnd2,
                relu,
                satfinite,
                f16x2,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF16x2F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rs")?;
            let rs = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
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
            let rbits = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRsReluSatfiniteF16x2F32 {
                rs,
                relu,
                satfinite,
                f16x2,
                f32,
                d,
                a,
                b,
                rbits,
            })
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteBf16F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd2 = Frnd2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtFrnd2ReluSatfiniteBf16F32 {
                frnd2,
                relu,
                satfinite,
                bf16,
                f32,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtFrnd2ReluSatfiniteBf16x2F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd2 = Frnd2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
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
            Ok(CvtFrnd2ReluSatfiniteBf16x2F32 {
                frnd2,
                relu,
                satfinite,
                bf16x2,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtRsReluSatfiniteBf16x2F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rs")?;
            let rs = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
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
            let rbits = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRsReluSatfiniteBf16x2F32 {
                rs,
                relu,
                satfinite,
                bf16x2,
                f32,
                d,
                a,
                b,
                rbits,
            })
        }
    }

    impl PtxParser for CvtRnaSatfiniteTf32F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rna")?;
            let rna = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf32 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnaSatfiniteTf32F32 {
                rna,
                satfinite,
                tf32,
                f32,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtFrnd2SatfiniteReluTf32F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd2 = Frnd2::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".tf32")?;
            let tf32 = ();
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtFrnd2SatfiniteReluTf32F32 {
                frnd2,
                satfinite,
                relu,
                tf32,
                f32,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF8x2typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let f8x2type = F8x2type::parse(stream)?;
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
            Ok(CvtRnSatfiniteReluF8x2typeF32 {
                rn,
                satfinite,
                relu,
                f8x2type,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF8x2typeF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let f8x2type = F8x2type::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnSatfiniteReluF8x2typeF16x2 {
                rn,
                satfinite,
                relu,
                f8x2type,
                f16x2,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRnReluF16x2F8x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let f8x2type = F8x2type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnReluF16x2F8x2type {
                rn,
                relu,
                f16x2,
                f8x2type,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF8x4typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rs")?;
            let rs = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let f8x4type = F8x4type::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let rbits = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRsReluSatfiniteF8x4typeF32 {
                rs,
                relu,
                satfinite,
                f8x4type,
                f32,
                d,
                a,
                rbits,
            })
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF4x2typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let f4x2type = F4x2type::parse(stream)?;
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
            Ok(CvtRnSatfiniteReluF4x2typeF32 {
                rn,
                satfinite,
                relu,
                f4x2type,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtRnReluF16x2F4x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let f4x2type = F4x2type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnReluF16x2F4x2type {
                rn,
                relu,
                f16x2,
                f4x2type,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF4x4typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rs")?;
            let rs = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let f4x4type = F4x4type::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let rbits = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRsReluSatfiniteF4x4typeF32 {
                rs,
                relu,
                satfinite,
                f4x4type,
                f32,
                d,
                a,
                rbits,
            })
        }
    }

    impl PtxParser for CvtRnSatfiniteReluF6x2typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let f6x2type = F6x2type::parse(stream)?;
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
            Ok(CvtRnSatfiniteReluF6x2typeF32 {
                rn,
                satfinite,
                relu,
                f6x2type,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtRnReluF16x2F6x2type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let f6x2type = F6x2type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnReluF16x2F6x2type {
                rn,
                relu,
                f16x2,
                f6x2type,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRsReluSatfiniteF6x4typeF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rs")?;
            let rs = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".satfinite")?;
            let satfinite = ();
            stream.expect_complete()?;
            let f6x4type = F6x4type::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = VectorOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let rbits = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRsReluSatfiniteF6x4typeF32 {
                rs,
                relu,
                satfinite,
                f6x4type,
                f32,
                d,
                a,
                rbits,
            })
        }
    }

    impl PtxParser for CvtFrnd3SatfiniteUe8m0x2F32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd3 = Frnd3::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".ue8m0x2")?;
            let ue8m0x2 = ();
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
            Ok(CvtFrnd3SatfiniteUe8m0x2F32 {
                frnd3,
                satfinite,
                ue8m0x2,
                f32,
                d,
                a,
                b,
            })
        }
    }

    impl PtxParser for CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            let frnd3 = Frnd3::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let satfinite = stream.expect_string(".satfinite").is_ok();
            if !satfinite {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".ue8m0x2")?;
            let ue8m0x2 = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
                frnd3,
                satfinite,
                ue8m0x2,
                bf16x2,
                d,
                a,
            })
        }
    }

    impl PtxParser for CvtRnBf16x2Ue8m0x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cvt")?;
            stream.expect_string(".rn")?;
            let rn = ();
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            stream.expect_complete()?;
            stream.expect_string(".ue8m0x2")?;
            let ue8m0x2 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CvtRnBf16x2Ue8m0x2 {
                rn,
                bf16x2,
                ue8m0x2,
                d,
                a,
            })
        }
    }
}
