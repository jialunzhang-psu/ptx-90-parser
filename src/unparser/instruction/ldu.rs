//! Original PTX specification:
//!
//! ldu{.ss}.type      d, [a];       // load from address
//! ldu{.ss}.vec.type  d, [a];       // vec load from address
//! .ss   = { .global };             // state space
//! .vec  = { .v2, .v4 };
//! .type = { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ldu::section_0::*;

    impl PtxUnparser for LduSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "ldu");
            if let Some(ss_0) = self.ss.as_ref() {
                match ss_0 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.type_ {
                Type::B128 => {
                    push_directive(tokens, "b128");
                }
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for LduSsVecType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "ldu");
            if let Some(ss_1) = self.ss.as_ref() {
                match ss_1 {
                    Ss::Global => {
                        push_directive(tokens, "global");
                    }
                }
            }
            match &self.vec {
                Vec::V2 => {
                    push_directive(tokens, "v2");
                }
                Vec::V4 => {
                    push_directive(tokens, "v4");
                }
            }
            match &self.type_ {
                Type::B128 => {
                    push_directive(tokens, "b128");
                }
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
