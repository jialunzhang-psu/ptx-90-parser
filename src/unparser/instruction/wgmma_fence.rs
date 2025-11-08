//! Original PTX specification:
//!
//! wgmma.fence.sync.aligned;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wgmma_fence::section_0::*;

    impl PtxUnparser for WgmmaFenceSyncAligned {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wgmma");
            push_directive(tokens, "fence");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            tokens.push(PtxToken::Semicolon);
        }
    }
}
