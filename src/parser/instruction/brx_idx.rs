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
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            let index = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let tlist = Operand::parse(stream)?;
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
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            let index = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let tlist = Operand::parse(stream)?;
            Ok(BrxIdxUni1 {
                idx,
                uni,
                index,
                tlist,
            })
        }
    }


}

