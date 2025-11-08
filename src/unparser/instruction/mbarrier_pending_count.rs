//! Original PTX specification:
//!
//! mbarrier.pending_count.b64 count, state;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_pending_count::section_0::*;

    impl PtxUnparser for MbarrierPendingCountB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "pending_count");
            push_directive(tokens, "b64");
            self.count.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.state.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
