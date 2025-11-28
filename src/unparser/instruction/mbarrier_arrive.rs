//! Original PTX specification:
//!
//! mbarrier.arrive{.sem}{.scope}{.state}.b64           state, [addr]{, count};
//! mbarrier.arrive{.sem}{.scope}{.shared::cluster}.b64         _, [addr] {,count};
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.state}.b64 state, [addr], txCount;
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.shared::cluster}.b64   _, [addr], txCount;
//! mbarrier.arrive.noComplete{.release}{.cta}{.state}.b64  state, [addr], count;
//! .sem   = { .release, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_arrive::section_0::*;

    impl PtxUnparser for MbarrierArriveSemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "arrive");
            if let Some(sem_0) = self.sem.as_ref() {
                match sem_0 {
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
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
            if let Some(state_2) = self.state.as_ref() {
                match state_2 {
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
            self.state2.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            if self.count.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_3) = self.count.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_3.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MbarrierArriveSemScopeSharedClusterB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "arrive");
            if let Some(sem_4) = self.sem.as_ref() {
                match sem_4 {
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
                    Sem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                }
            }
            if let Some(scope_5) = self.scope.as_ref() {
                match scope_5 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                }
            }
            if self.shared_cluster {
                push_directive(tokens, "shared::cluster");
            }
            push_directive(tokens, "b64");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.operand.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            if self.count.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_6) = self.count.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_6.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for MbarrierArriveExpectTxSemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "arrive");
            push_directive(tokens, "expect_tx");
            if let Some(sem_7) = self.sem.as_ref() {
                match sem_7 {
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
                    Sem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                }
            }
            if let Some(scope_8) = self.scope.as_ref() {
                match scope_8 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                }
            }
            if let Some(state_9) = self.state.as_ref() {
                match state_9 {
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
            self.state2.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
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

    impl PtxUnparser for MbarrierArriveExpectTxSemScopeSharedClusterB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "arrive");
            push_directive(tokens, "expect_tx");
            if let Some(sem_10) = self.sem.as_ref() {
                match sem_10 {
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
                    Sem::Relaxed => {
                        push_directive(tokens, "relaxed");
                    }
                }
            }
            if let Some(scope_11) = self.scope.as_ref() {
                match scope_11 {
                    Scope::Cluster => {
                        push_directive(tokens, "cluster");
                    }
                    Scope::Cta => {
                        push_directive(tokens, "cta");
                    }
                }
            }
            if self.shared_cluster {
                push_directive(tokens, "shared::cluster");
            }
            push_directive(tokens, "b64");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.operand.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
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

    impl PtxUnparser for MbarrierArriveNocompleteReleaseCtaStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "mbarrier");
            push_directive(tokens, "arrive");
            push_directive(tokens, "noComplete");
            if self.release {
                push_directive(tokens, "release");
            }
            if self.cta {
                push_directive(tokens, "cta");
            }
            if let Some(state_12) = self.state.as_ref() {
                match state_12 {
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
            self.state2.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.count.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
