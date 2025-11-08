//! Original PTX specification:
//!
//! dp2a.mode.atype.btype  d, a, b, c;
//! .atype = .btype = { .u32, .s32 };
//! .mode = { .lo, .hi };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::dp2a::section_0::*;

    impl PtxUnparser for Dp2aModeAtypeBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "dp2a");
            match &self.mode {
                Mode::Lo => {
                    push_directive(tokens, "lo");
                }
                Mode::Hi => {
                    push_directive(tokens, "hi");
                }
            }
            match &self.atype {
                Atype::U32 => {
                    push_directive(tokens, "u32");
                }
                Atype::S32 => {
                    push_directive(tokens, "s32");
                }
            }
            match &self.btype {
                Btype::U32 => {
                    push_directive(tokens, "u32");
                }
                Btype::S32 => {
                    push_directive(tokens, "s32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
