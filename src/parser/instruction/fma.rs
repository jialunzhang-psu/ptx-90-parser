//! Original PTX specification:
//!
//! fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
//! fma.rnd{.ftz}.f32x2      d, a, b, c;
//! fma.rnd.f64              d, a, b, c;
//! .rnd = { .rn, .rz, .rm, .rp };
//! ---------------------------------------------
//! fma.rnd{.ftz}{.sat}.f16     d, a, b, c;
//! fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
//! fma.rnd{.ftz}.relu.f16      d, a, b, c;
//! fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
//! fma.rnd{.relu}.bf16         d, a, b, c;
//! fma.rnd{.relu}.bf16x2       d, a, b, c;
//! fma.rnd.oob{.relu}.type     d, a, b, c;
//! .rnd = { .rn };
//! ---------------------------------------------
//! fma.rnd{.sat}.f32.abtype  d, a, b, c;
//! .abtype = { .f16, .bf16};
//! .rnd    = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::fma::section_0::*;

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

    impl PtxParser for FmaRndFtzSatF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
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
            Ok(FmaRndFtzSatF32 {
                rnd,
                ftz,
                sat,
                f32,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndFtzF32x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f32x2")?;
            let f32x2 = ();
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
            Ok(FmaRndFtzF32x2 {
                rnd,
                ftz,
                f32x2,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndF64 {
                rnd,
                f64,
                d,
                a,
                b,
                c,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::fma::section_1::*;

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

    impl PtxParser for FmaRndFtzSatF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndFtzSatF16 {
                rnd,
                ftz,
                sat,
                f16,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndFtzSatF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndFtzSatF16x2 {
                rnd,
                ftz,
                sat,
                f16x2,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndFtzReluF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".relu")?;
            let relu = ();
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndFtzReluF16 {
                rnd,
                ftz,
                relu,
                f16,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndFtzReluF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".relu")?;
            let relu = ();
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndFtzReluF16x2 {
                rnd,
                ftz,
                relu,
                f16x2,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndReluBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndReluBf16 {
                rnd,
                relu,
                bf16,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndReluBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndReluBf16x2 {
                rnd,
                relu,
                bf16x2,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for FmaRndOobReluType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".oob")?;
            let oob = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let relu = stream.expect_string(".relu").is_ok();
            if !relu {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".type")?;
            let type_ = ();
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
            Ok(FmaRndOobReluType {
                rnd,
                oob,
                relu,
                type_,
                d,
                a,
                b,
                c,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::fma::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Abtype::Bf16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Abtype::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16", ".f16"];
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

    impl PtxParser for FmaRndSatF32Abtype {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fma")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let sat = stream.expect_string(".sat").is_ok();
            if !sat {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
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
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FmaRndSatF32Abtype {
                rnd,
                sat,
                f32,
                abtype,
                d,
                a,
                b,
                c,
            })
        }
    }


}

