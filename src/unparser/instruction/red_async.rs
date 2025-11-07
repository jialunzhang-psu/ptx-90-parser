//! Original PTX specification:
//!
//! // Increment and Decrement reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  =                 { .relaxed };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .op   =                 { .inc, .dec };
//! .type =                 { .u32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // MIN and MAX reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .min, .max };
//! .type = { .u32, .s32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // Bitwise AND, OR and XOR reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .and, .or, .xor };
//! .type = { .b32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // ADD reductions
//! red.async.sem.scope{.ss}.completion_mechanism.add.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .type = { .u32, .s32, .u64 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------
//! // Alternate floating point type:
//! red.async{.mmio}.sem.scope{.ss}.add.type [a], b;
//! .sem  = { .release };
//! .scope = { .gpu, .cluster };
//! .ss   = { .global };
//! .type = { .u32, .s32, .u64, .s64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::red_async::section_0::*;

    impl PtxUnparser for RedAsyncSemScopeSsCompletionMechanismOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "async");
                    match &self.sem {
                            Sem::Relaxed => {
                                    push_directive(tokens, "relaxed");
                            }
                    }
                    match &self.scope {
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    match &self.op {
                            Op::Inc => {
                                    push_directive(tokens, "inc");
                            }
                            Op::Dec => {
                                    push_directive(tokens, "dec");
                            }
                    }
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::red_async::section_1::*;

    impl PtxUnparser for RedAsyncSemScopeSsCompletionMechanismOpType1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "async");
                    match &self.sem {
                            Sem::Relaxed => {
                                    push_directive(tokens, "relaxed");
                            }
                    }
                    match &self.scope {
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                    }
                    if let Some(ss_1) = self.ss.as_ref() {
                            match ss_1 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    match &self.op {
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::red_async::section_2::*;

    impl PtxUnparser for RedAsyncSemScopeSsCompletionMechanismOpType2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "async");
                    match &self.sem {
                            Sem::Relaxed => {
                                    push_directive(tokens, "relaxed");
                            }
                    }
                    match &self.scope {
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                    }
                    if let Some(ss_2) = self.ss.as_ref() {
                            match ss_2 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    match &self.op {
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Xor => {
                                    push_directive(tokens, "xor");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::red_async::section_3::*;

    impl PtxUnparser for RedAsyncSemScopeSsCompletionMechanismAddType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "async");
                    match &self.sem {
                            Sem::Relaxed => {
                                    push_directive(tokens, "relaxed");
                            }
                    }
                    match &self.scope {
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                    }
                    if let Some(ss_3) = self.ss.as_ref() {
                            match ss_3 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    push_directive(tokens, "add");
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::red_async::section_4::*;

    impl PtxUnparser for RedAsyncMmioSemScopeSsAddType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "red");
                    push_directive(tokens, "async");
                    if self.mmio {
                            push_directive(tokens, "mmio");
                    }
                    match &self.sem {
                            Sem::Release => {
                                    push_directive(tokens, "release");
                            }
                    }
                    match &self.scope {
                            Scope::Cluster => {
                                    push_directive(tokens, "cluster");
                            }
                            Scope::Gpu => {
                                    push_directive(tokens, "gpu");
                            }
                    }
                    if let Some(ss_4) = self.ss.as_ref() {
                            match ss_4 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                            }
                    }
                    push_directive(tokens, "add");
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

