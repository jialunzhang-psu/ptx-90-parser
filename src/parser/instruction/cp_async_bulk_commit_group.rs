//! Original PTX specification:
//!
//! cp.async.bulk.commit_group;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_commit_group::section_0::*;

    impl PtxParser for CpAsyncBulkCommitGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_complete()?;
            stream.expect_string(".commit_group")?;
            let commit_group = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CpAsyncBulkCommitGroup {
                async_,
                bulk,
                commit_group,
            })
        }
    }
}
