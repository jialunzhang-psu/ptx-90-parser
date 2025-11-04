//! Original PTX specification:
//!
//! rsqrt.approx{.ftz}.f32  d, a;
//! rsqrt.approx.f64        d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::rsqrt::section_0::*;

    impl PtxParser for RsqrtApproxFtzF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("rsqrt")?;
            stream.expect_string(".approx")?;
            let approx = ();
            let saved_pos = stream.position();
            let ftz = stream.expect_string(".ftz").is_ok();
            if !ftz {
                stream.set_position(saved_pos);
            }
            stream.expect_string(".f32")?;
            let f32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            Ok(RsqrtApproxFtzF32 {
                approx,
                ftz,
                f32,
                d,
                a,
            })
        }
    }


    impl PtxParser for RsqrtApproxF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("rsqrt")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_string(".f64")?;
            let f64 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let a = Operand::parse(stream)?;
            Ok(RsqrtApproxF64 {
                approx,
                f64,
                d,
                a,
            })
        }
    }


}

