//! Original PTX specification:
//!
//! copysign.type  d, a, b;
//! .type = { .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::copysign::section_0::*;

    impl PtxUnparser for CopysignType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "copysign");
                    match &self.type_ {
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
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

