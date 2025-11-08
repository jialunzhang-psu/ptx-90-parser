//! Original PTX specification:
//!
//! mbarrier.init{.state}.b64 [addr], count;
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_init::section_0::*;

    impl PtxUnparser for MbarrierInitStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "init");
            if let Some(state_0) = self.state.as_ref() {
                match state_0 {
                    State::SharedCta => {
                        push_directive(tokens, "shared::cta");
                    }
                    State::Shared => {
                        push_directive(tokens, "shared");
                    }
                }
            }
            push_directive(tokens, "b64");
            self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.count.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
