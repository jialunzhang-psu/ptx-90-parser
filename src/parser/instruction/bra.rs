//! Original PTX specification:
//!
//! bra{.uni}  tgt;           // tgt is a label
//! bra{.uni}  tgt;           // unconditional branch

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bra::section_0::*;

    impl PtxParser for BraUni {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bra")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let tgt = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BraUni {
                uni,
                tgt,
            })
        }
    }


    impl PtxParser for BraUni1 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bra")?;
            let saved_pos = stream.position();
            let uni = stream.expect_string(".uni").is_ok();
            if !uni {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let tgt = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(BraUni1 {
                uni,
                tgt,
            })
        }
    }


}

