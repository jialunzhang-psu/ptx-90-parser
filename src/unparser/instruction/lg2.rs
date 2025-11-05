//! Original PTX specification:
//!
//! lg2.approx{.ftz}.f32  d, a;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::lg2::section_0::*;

    impl PtxUnparser for Lg2ApproxFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "lg2");
                    push_directive(tokens, "approx");
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

}

