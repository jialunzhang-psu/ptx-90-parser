//! Original PTX specification:
//!
//! cp.async.commit_group ;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_commit_group::section_0::*;

    impl PtxParser for CpAsyncCommitGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            stream.expect_string(".commit_group")?;
            let commit_group = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(CpAsyncCommitGroup {
                async_,
                commit_group,
            })
        }
    }


}

