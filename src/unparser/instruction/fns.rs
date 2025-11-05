//! Original PTX specification:
//!
//! fns.b32 d, mask, base, offset;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::fns::section_0::*;

    impl PtxUnparser for FnsB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fns");
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.mask.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.base.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.offset.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

