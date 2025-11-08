//! Original PTX specification:
//!
//! abs.type  d, a;
//! .type = { .s16, .s32, .s64 };
//!
//! abs{.ftz}.f32  d, a;
//! abs.f64        d, a;
//!
//! abs{.ftz}.f16    d, a;
//! abs{.ftz}.f16x2  d, a;
//! abs.bf16         d, a;
//! abs.bf16x2       d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::abs::section_0::*;

    impl PtxUnparser for AbsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            match &self.type_ {
                Type::S16 => {
                    push_directive(tokens, "s16");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::S64 => {
                    push_directive(tokens, "s64");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            push_directive(tokens, "f64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsFtzF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsFtzF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            if self.ftz {
                push_directive(tokens, "ftz");
            }
            push_directive(tokens, "f16x2");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            push_directive(tokens, "bf16");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for AbsBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "abs");
            push_directive(tokens, "bf16x2");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
