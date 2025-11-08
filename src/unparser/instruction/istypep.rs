//! Original PTX specification:
//!
//! istypep.type   p, a;  // result is .pred
//! .type = { .texref, .samplerref, .surfref };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::istypep::section_0::*;

    impl PtxUnparser for IstypepType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "istypep");
            match &self.type_ {
                Type::Samplerref => {
                    push_directive(tokens, "samplerref");
                }
                Type::Surfref => {
                    push_directive(tokens, "surfref");
                }
                Type::Texref => {
                    push_directive(tokens, "texref");
                }
            }
            self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
