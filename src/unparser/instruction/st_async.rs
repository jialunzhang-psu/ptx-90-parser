//! Original PTX specification:
//!
//! st.async{.sem}{.scope}{.ss}{.completion_mechanism}{.vec}.type [a], b, [mbar];
//! .sem  =                 { .weak };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .type =                 { .b32, .b64,
//! .u32, .u64,
//! .s32, .s64,
//! .f32, .f64 };
//! .vec  =                 { .v2, .v4 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ---------------------------------------------------------
//! st.async{.mmio}.sem.scope{.ss}.type [a], b;
//! .sem =                  { .release };
//! .scope =                { .gpu, .sys };
//! .ss =                   { .global };
//! .type =                 { .b8, .b16, .b32, .b64,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::st_async::section_0::*;

    impl PtxUnparser for StAsyncSemScopeSsCompletionMechanismVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "st");
                    push_directive(tokens, "async");
                    if let Some(sem_0) = self.sem.as_ref() {
                            match sem_0 {
                                    Sem::Weak => {
                                            push_directive(tokens, "weak");
                                    }
                            }
                    }
                    if let Some(scope_1) = self.scope.as_ref() {
                            match scope_1 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                            }
                    }
                    if let Some(ss_2) = self.ss.as_ref() {
                            match ss_2 {
                                    Ss::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    if let Some(completion_mechanism_3) = self.completion_mechanism.as_ref() {
                            match completion_mechanism_3 {
                                    CompletionMechanism::MbarrierCompleteTxBytes => {
                                            push_directive(tokens, "mbarrier::complete_tx::bytes");
                                    }
                            }
                    }
                    if let Some(vec_4) = self.vec.as_ref() {
                            match vec_4 {
                                    Vec::V2 => {
                                            push_directive(tokens, "v2");
                                    }
                                    Vec::V4 => {
                                            push_directive(tokens, "v4");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
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
    use crate::r#type::instruction::st_async::section_1::*;

    impl PtxUnparser for StAsyncMmioSemScopeSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "st");
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
                            Scope::Gpu => {
                                    push_directive(tokens, "gpu");
                            }
                            Scope::Sys => {
                                    push_directive(tokens, "sys");
                            }
                    }
                    if let Some(ss_5) = self.ss.as_ref() {
                            match ss_5 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Type::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Type::S8 => {
                                    push_directive(tokens, "s8");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

