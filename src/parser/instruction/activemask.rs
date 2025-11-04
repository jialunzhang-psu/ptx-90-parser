//! Original PTX specification:
//!
//! activemask.b32 d;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::activemask::section_0::*;

    impl PtxParser for ActivemaskB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("activemask")?;
            stream.expect_string(".b32")?;
            let b32 = ();
            let d = Operand::parse(stream)?;
            Ok(ActivemaskB32 {
                b32,
                d,
            })
        }
    }


}

