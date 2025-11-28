//! Original PTX specification:
//!
//! madc.hilo{.cc}.type  d, a, b, c;
//! .type = { .u32, .s32, .u64, .s64 };
//! .hilo = { .hi, .lo };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::madc::section_0::*;

    impl PtxUnparser for MadcHiloCcType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "madc");
            match &self.hilo {
                Hilo::Hi => {
                    push_directive(tokens, "hi");
                }
                Hilo::Lo => {
                    push_directive(tokens, "lo");
                }
            }
            if self.cc {
                push_directive(tokens, "cc");
            }
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
