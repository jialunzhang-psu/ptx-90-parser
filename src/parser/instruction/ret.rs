//! Original PTX specification:
//!
//! ret{.uni};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ret::section_0::*;

    impl PtxParser for RetUni {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("ret")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(RetUni { uni })
        }
    }
}
