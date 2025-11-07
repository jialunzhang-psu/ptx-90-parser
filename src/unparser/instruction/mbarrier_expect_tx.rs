//! Original PTX specification:
//!
//! mbarrier.expect_tx{.sem}{.scope}{.space}.b64 [addr], txCount;
//! .sem   = { .relaxed };
//! .scope = { .cta, .cluster };
//! .space = { .shared, .shared::cta, .shared::cluster };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_expect_tx::section_0::*;

    impl PtxUnparser for MbarrierExpectTxSemScopeSpaceB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "expect_tx");
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
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.txcount.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

