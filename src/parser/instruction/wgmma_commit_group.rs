//! Original PTX specification:
//!
//! wgmma.commit_group.sync.aligned;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_commit_group::section_0::*;

    impl PtxParser for WgmmaCommitGroupSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("wgmma")?;
            stream.expect_string(".commit_group")?;
            let commit_group = ();
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(WgmmaCommitGroupSyncAligned {
                commit_group,
                sync,
                aligned,
            })
        }
    }
}
