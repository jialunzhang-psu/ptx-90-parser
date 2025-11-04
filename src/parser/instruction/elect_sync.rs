//! Original PTX specification:
//!
//! elect.sync d|p, membermask;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::elect_sync::section_0::*;

    impl PtxParser for ElectSync {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("elect")?;
            stream.expect_string(".sync")?;
            let sync = ();
            let d = Operand::parse(stream)?;
            stream.expect(&PtxToken::Pipe)?;
            let p = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let membermask = Operand::parse(stream)?;
            Ok(ElectSync {
                sync,
                d,
                membermask,
            })
        }
    }


}

