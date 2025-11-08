//! Original PTX specification:
//!
//! tcgen05.fence::before_thread_sync ;
//! tcgen05.fence::after_thread_sync  ;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_fence::section_0::*;

    impl PtxUnparser for Tcgen05FenceBeforeThreadSync {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "fence::before_thread_sync");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05FenceAfterThreadSync {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
            push_directive(tokens, "fence::after_thread_sync");
            tokens.push(PtxToken::Semicolon);
        }
    }
}
