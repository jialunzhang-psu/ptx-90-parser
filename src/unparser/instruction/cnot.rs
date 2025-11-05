//! Original PTX specification:
//!
//! cnot.type d, a;
//! .type = { .b16, .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cnot::section_0::*;

    impl PtxUnparser for CnotType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cnot");
                    match &self.type_ {
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

