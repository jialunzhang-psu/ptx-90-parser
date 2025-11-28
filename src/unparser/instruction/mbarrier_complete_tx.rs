//! Original PTX specification:
//!
//! mbarrier.complete_tx{.sem}{.scope}{.space}.b64 [addr], txCount;
//! .sem   = { .relaxed };
//! .scope = { .cta, .cluster };
//! .space = { .shared, .shared::cta, .shared::cluster };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_complete_tx::section_0::*;

    impl PtxUnparser for MbarrierCompleteTxSemScopeSpaceB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "complete_tx");
            if let Some(sem_0) = self.sem.as_ref() {
                match sem_0 {
                    Sem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                }
            }
            if let Some(scope_1) = self.scope.as_ref() {
                match scope_1 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                }
            }
            if let Some(space_2) = self.space.as_ref() {
                match space_2 {
                    Space::SharedCluster => {
                        push_directive(tokens, "shared::cluster");
                    }
                    Space::SharedCta => {
                        push_directive(tokens, "shared::cta");
                    }
                    Space::Shared => {
                        push_directive(tokens, "shared");
                    }
                }
            }
            push_directive(tokens, "b64");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.txcount.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
