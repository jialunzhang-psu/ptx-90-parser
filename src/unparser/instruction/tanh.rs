//! Original PTX specification:
//!
//! tanh.approx.type d, a;
//! .type = {.f16, .f32, .f16x2, .bf16, .bf16x2};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tanh::section_0::*;

    impl PtxUnparser for TanhApproxType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tanh");
            push_directive(tokens, "approx");
            match &self.type_ {
                Type::Bf16x2 => {
                    push_directive(tokens, "bf16x2");
                }
                Type::F16x2 => {
                    push_directive(tokens, "f16x2");
                }
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
                }
                Type::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
