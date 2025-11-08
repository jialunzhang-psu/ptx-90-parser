//! Original PTX specification:
//!
//! brx.idx{.uni} index, tlist;
//! brx.idx{.uni} index, tlist;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::brx_idx::section_0::*;

    impl PtxParser for BrxIdxUni {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("brx")?;
            stream.expect_string(".idx")?;
            let idx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let index = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tlist = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BrxIdxUni {
                idx,
                uni,
                index,
                tlist,
            })
        }
    }

    impl PtxParser for BrxIdxUni1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("brx")?;
            stream.expect_string(".idx")?;
            let idx = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let index = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tlist = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BrxIdxUni1 {
                idx,
                uni,
                index,
                tlist,
            })
        }
    }
}
