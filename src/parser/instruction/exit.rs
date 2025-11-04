//! Original PTX specification:
//!
//! exit;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::exit::section_0::*;

    impl PtxParser for Exit {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("exit")?;
            Ok(Exit {
            })
        }
    }


}

