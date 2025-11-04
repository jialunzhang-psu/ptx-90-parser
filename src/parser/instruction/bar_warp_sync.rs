//! Original PTX specification:
//!
//! bar.warp.sync      membermask;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bar_warp_sync::section_0::*;

    impl PtxParser for BarWarpSync {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("bar")?;
            stream.expect_string(".warp")?;
            let warp = ();
            stream.expect_string(".sync")?;
            let sync = ();
            let membermask = Operand::parse(stream)?;
            Ok(BarWarpSync {
                warp,
                sync,
                membermask,
            })
        }
    }


}

