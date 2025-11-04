//! Original PTX specification:
//!
//! lg2.approx{.ftz}.f32  d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::lg2::section_0::*;

    impl PtxParser for Lg2ApproxFtzF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("lg2")?;
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
            Ok(Lg2ApproxFtzF32 {
                approx,
                ftz,
                f32,
                d,
                a,
            })
        }
    }


}

