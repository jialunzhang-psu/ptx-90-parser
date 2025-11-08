//! Original PTX specification:
//!
//! cp.async.wait_group N;
//! cp.async.wait_all ;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_wait_group::section_0::*;

    impl PtxUnparser for CpAsyncWaitGroup {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "wait_group");
            self.n.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CpAsyncWaitAll {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "wait_all");
            tokens.push(PtxToken::Semicolon);
        }
    }
}
