//! Original PTX specification:
//!
//! trap;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::trap::section_0::*;

    impl PtxParser for Trap {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("trap")?;
            Ok(Trap {
            })
        }
    }


}

