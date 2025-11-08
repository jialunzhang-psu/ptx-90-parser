//! Original PTX specification:
//!
//! subc{.cc}.type  d, a, b;
//! .type = { .u32, .s32, .u64, .s64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::subc::section_0::*;

    impl PtxUnparser for SubcCcType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "subc");
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
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
