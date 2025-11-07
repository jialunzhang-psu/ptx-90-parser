//! Original PTX specification:
//!
//! mbarrier.test_wait{.sem}{.scope}{.state}.b64        waitComplete, [addr], state;
//! mbarrier.test_wait.parity{.sem}{.scope}{.state}.b64 waitComplete, [addr], phaseParity;
//! mbarrier.try_wait{.sem}{.scope}{.state}.b64         waitComplete, [addr], state {, suspendTimeHint};
//! mbarrier.try_wait.parity{.sem}{.scope}{.state}.b64  waitComplete, [addr], phaseParity {, suspendTimeHint};
//! .sem   = { .acquire, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mbarrier_test_wait::section_0::*;

    impl PtxUnparser for MbarrierTestWaitSemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "test_wait");
                    if let Some(sem_0) = self.sem.as_ref() {
                            match sem_0 {
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
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
                    self.waitcomplete.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.state2.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierTestWaitParitySemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "test_wait");
                    push_directive(tokens, "parity");
                    if let Some(sem_3) = self.sem.as_ref() {
                            match sem_3 {
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                            }
                    }
                    if let Some(scope_4) = self.scope.as_ref() {
                            match scope_4 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                            }
                    }
                    if let Some(state_5) = self.state.as_ref() {
                            match state_5 {
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.waitcomplete.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.phaseparity.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierTryWaitSemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "try_wait");
                    if let Some(sem_6) = self.sem.as_ref() {
                            match sem_6 {
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                            }
                    }
                    if let Some(scope_7) = self.scope.as_ref() {
                            match scope_7 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                            }
                    }
                    if let Some(state_8) = self.state.as_ref() {
                            match state_8 {
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.waitcomplete.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.state2.unparse_tokens(tokens);
            if self.suspendtimehint.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_9) = self.suspendtimehint.as_ref() {
                        opt_9.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MbarrierTryWaitParitySemScopeStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mbarrier");
                    push_directive(tokens, "try_wait");
                    push_directive(tokens, "parity");
                    if let Some(sem_10) = self.sem.as_ref() {
                            match sem_10 {
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
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
                    self.waitcomplete.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.phaseparity.unparse_tokens(tokens);
            if self.suspendtimehint.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_13) = self.suspendtimehint.as_ref() {
                        opt_13.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

