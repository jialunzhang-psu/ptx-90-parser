//! Original PTX specification:
//!
//! fns.b32 d, mask, base, offset;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::fns::section_0::*;

    impl PtxParser for FnsB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fns")?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let mask = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let base = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let offset = Operand::parse(stream)?;
            Ok(FnsB32 {
                b32,
                d,
                mask,
                base,
                offset,
            })
        }
    }


}

