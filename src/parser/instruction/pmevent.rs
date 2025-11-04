//! Original PTX specification:
//!
//! pmevent       a;    // trigger a single performance monitor event
//! pmevent.mask  a;    // trigger one or more performance monitor events

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::pmevent::section_0::*;

    impl PtxParser for Pmevent {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("pmevent")?;
            let a = Operand::parse(stream)?;
            Ok(Pmevent {
                a,
            })
        }
    }


    impl PtxParser for PmeventMask {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("pmevent")?;
            stream.expect_string(".mask")?;
            let mask = ();
            let a = Operand::parse(stream)?;
            Ok(PmeventMask {
                mask,
                a,
            })
        }
    }


}

