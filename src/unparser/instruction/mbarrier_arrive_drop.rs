//! Original PTX specification:
//!
//! mbarrier.arrive_drop{.sem}{.scope}{.state}.b64 state,           [addr]{, count};
//! mbarrier.arrive_drop{.sem}{.scope}{.shared::cluster}.b64           _,   [addr] {,count};
//! mbarrier.arrive_drop.expect_tx{.state}{.sem}{.scope}.b64 state, [addr], tx_count;
//! mbarrier.arrive_drop.expect_tx{.shared::cluster}{.sem}{.scope}.b64   _, [addr], tx_count;
//! mbarrier.arrive_drop.noComplete{.release}{.cta}{.state}.b64 state,  [addr], count;
//! .sem   = { .release, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_arrive_drop::section_0::*;

    impl PtxUnparser for MbarrierArriveDropSemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "arrive_drop");
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
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                            }
                    }
                    if let Some(state_2) = self.state.as_ref() {
                            match state_2 {
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.state2.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            if self.count.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.count.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierArriveDropSemScopeSharedClusterB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "arrive_drop");
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
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                            }
                    }
                    if self.shared_cluster {
                            push_directive(tokens, "shared::cluster");
                    }
                    push_directive(tokens, "b64");
                    self.operand.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            if self.count.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_6) = self.count.as_ref() {
                        opt_6.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierArriveDropExpectTxStateSemScopeB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "arrive_drop");
                    push_directive(tokens, "expect_tx");
                    if let Some(state_7) = self.state.as_ref() {
                            match state_7 {
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    if let Some(sem_8) = self.sem.as_ref() {
                            match sem_8 {
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                            }
                    }
                    if let Some(scope_9) = self.scope.as_ref() {
                            match scope_9 {
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.state2.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.tx_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierArriveDropExpectTxSharedClusterSemScopeB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "arrive_drop");
                    push_directive(tokens, "expect_tx");
                    if self.shared_cluster {
                            push_directive(tokens, "shared::cluster");
                    }
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
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.operand.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.tx_count.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierArriveDropNocompleteReleaseCtaStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "arrive_drop");
                    push_directive(tokens, "noComplete");
                    if self.release {
                            push_directive(tokens, "release");
                    }
                    if self.cta {
                            push_directive(tokens, "cta");
                    }
                    if let Some(state_12) = self.state.as_ref() {
                            match state_12 {
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.state2.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.count.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

