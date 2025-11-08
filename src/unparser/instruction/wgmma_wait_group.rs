//! Original PTX specification:
//!
//! wgmma.wait_group.sync.aligned N;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_wait_group::section_0::*;

    impl PtxUnparser for WgmmaWaitGroupSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wgmma");
            push_directive(tokens, "wait_group");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            self.n.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
