//! Original PTX specification:
//!
//! activemask.b32 d;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::activemask::section_0::*;

    impl PtxUnparser for ActivemaskB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "activemask");
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

