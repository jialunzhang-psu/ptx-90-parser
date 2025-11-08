//! Original PTX specification:
//!
//! mov.type  d, a;
//! // mov.type  d, sreg;
//! // mov.type  d, avar;       // get address of variable
//! // mov.type  d, avar+imm;   // get address of variable with offset
//! mov.u32   d, fname;      // get address of device function
//! mov.u64   d, fname;      // get address of device function
//! mov.u32   d, kernel;     // get address of entry function
//! mov.u64   d, kernel;     // get address of entry function
//! .type = { .pred,
//! .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };
//! ----------------------------------------------
//! mov.type  d, a;
//! .type = { .b16, .b32, .b64, .b128 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mov::section_0::*;

    impl PtxUnparser for MovType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
            match &self.type_ {
                Type::Pred => {
                    push_directive(tokens, "pred");
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
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MovU32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
            push_directive(tokens, "u32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.fname.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MovU64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
            push_directive(tokens, "u64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.fname.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MovU321 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
            push_directive(tokens, "u32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.kernel.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MovU641 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
            push_directive(tokens, "u64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.kernel.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mov::section_1::*;

    impl PtxUnparser for MovType1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mov");
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
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
