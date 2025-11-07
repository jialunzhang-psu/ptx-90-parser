//! Original PTX specification:
//!
//! sqrt.approx{.ftz}.f32  d, a; // fast, approximate square root
//! sqrt.rnd{.ftz}.f32     d, a; // IEEE 754 compliant rounding
//! sqrt.rnd.f64           d, a; // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sqrt::section_0::*;

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

    impl PtxParser for SqrtApproxFtzF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sqrt")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SqrtApproxFtzF32 {
                approx,
                ftz,
                f32,
                d,
                a,
            })
        }
    }


    impl PtxParser for SqrtRndFtzF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sqrt")?;
            let rnd = Rnd::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SqrtRndFtzF32 {
                rnd,
                ftz,
                f32,
                d,
                a,
            })
        }
    }


    impl PtxParser for SqrtRndF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("sqrt")?;
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
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(SqrtRndF64 {
                rnd,
                f64,
                d,
                a,
            })
        }
    }


}

