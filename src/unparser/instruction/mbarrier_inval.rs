//! Original PTX specification:
//!
//! mbarrier.inval{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_inval::section_0::*;

    impl PtxUnparser for MbarrierInvalStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "inval");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
