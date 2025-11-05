//! Original PTX specification:
//!
//! popc.type  d, a;
//! .type = { .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::popc::section_0::*;

    impl PtxUnparser for PopcType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "popc");
                    match &self.type_ {
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

