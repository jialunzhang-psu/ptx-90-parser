//! Original PTX specification:
//!
//! dp4a.atype.btype  d, a, b, c;
//! .atype = .btype = { .u32, .s32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::dp4a::section_0::*;

    impl PtxUnparser for Dp4aAtypeBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "dp4a");
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
