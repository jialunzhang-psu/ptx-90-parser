//! Original PTX specification:
//!
//! rsqrt.approx.ftz.f64 d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::rsqrt_approx_ftz_f64::section_0::*;

    impl PtxParser for RsqrtApproxFtzF64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("rsqrt")?;
            stream.expect_string(".approx")?;
            let approx = ();
            stream.expect_complete()?;
            stream.expect_string(".ftz")?;
            let ftz = ();
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
            Ok(RsqrtApproxFtzF64 {
                approx,
                ftz,
                f64,
                d,
                a,
            })
        }
    }
}
