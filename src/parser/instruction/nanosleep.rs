//! Original PTX specification:
//!
//! nanosleep.u32 t;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::nanosleep::section_0::*;

    impl PtxParser for NanosleepU32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("nanosleep")?;
            stream.expect_string(".u32")?;
            let u32 = ();
            let t = Operand::parse(stream)?;
            Ok(NanosleepU32 {
                u32,
                t,
            })
        }
    }


}

