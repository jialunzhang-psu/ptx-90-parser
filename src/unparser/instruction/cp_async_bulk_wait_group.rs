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
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "bulk");
                    push_directive(tokens, "wait_group");
                    if self.read {
                            push_directive(tokens, "read");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.n.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

