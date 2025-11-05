//! Original PTX specification:
//!
//! cp.async.bulk.commit_group;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_commit_group::section_0::*;

    impl PtxUnparser for CpAsyncBulkCommitGroup {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    push_directive(tokens, "commit_group");
            tokens.push(PtxToken::Semicolon);
        }
    }

}

