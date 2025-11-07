//! Original PTX specification:
//!
//! redux.sync.op.type dst, src, membermask;
//! .op   = {.add, .min, .max};
//! .type = {.u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op.b32 dst, src, membermask;
//! .op   = {.and, .or, .xor};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op{.abs}{.NaN}.f32 dst, src, membermask;
//! .op   = { .min, .max };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::redux_sync::section_0::*;

    impl PtxUnparser for ReduxSyncOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "redux");
                    push_directive(tokens, "sync");
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
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
                    self.dst.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.src.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::redux_sync::section_1::*;

    impl PtxUnparser for ReduxSyncOpB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "redux");
                    push_directive(tokens, "sync");
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
                    push_directive(tokens, "b32");
                    self.dst.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.src.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::redux_sync::section_2::*;

    impl PtxUnparser for ReduxSyncOpAbsNanF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "redux");
                    push_directive(tokens, "sync");
                    match &self.op {
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    if self.abs {
                            push_directive(tokens, "abs");
                    }
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    push_directive(tokens, "f32");
                    self.dst.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.src.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

