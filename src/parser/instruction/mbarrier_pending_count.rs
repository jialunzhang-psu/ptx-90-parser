//! Original PTX specification:
//!
//! mbarrier.pending_count.b64 count, state;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_pending_count::section_0::*;

    impl PtxParser for MbarrierPendingCountB64 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("mbarrier")?;
            stream.expect_string(".pending_count")?;
            let pending_count = ();
            stream.expect_string(".b64")?;
            let b64 = ();
            let count = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let state = Operand::parse(stream)?;
            Ok(MbarrierPendingCountB64 {
                pending_count,
                b64,
                count,
                state,
            })
        }
    }


}

