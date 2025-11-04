//! Original PTX specification:
//!
//! wgmma.fence.sync.aligned;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_fence::section_0::*;

    impl PtxParser for WgmmaFenceSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".fence")?;
            let fence = ();
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            Ok(WgmmaFenceSyncAligned {
                fence,
                sync,
                aligned,
            })
        }
    }


}

