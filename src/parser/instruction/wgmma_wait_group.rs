//! Original PTX specification:
//!
//! wgmma.wait_group.sync.aligned N;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_wait_group::section_0::*;

    impl PtxParser for WgmmaWaitGroupSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".wait_group")?;
            let wait_group = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let n = Operand::parse(stream)?;
            Ok(WgmmaWaitGroupSyncAligned {
                wait_group,
                sync,
                aligned,
                n,
            })
        }
    }


}

