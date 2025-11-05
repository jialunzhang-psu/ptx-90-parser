//! Original PTX specification:
//!
//! cp.async.bulk.wait_group{.read} N;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_wait_group::section_0::*;

    impl PtxUnparser for CpAsyncBulkWaitGroupRead {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    push_directive(tokens, "wait_group");
                    if self.read {
                            push_directive(tokens, "read");
                    }
                    self.n.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

