//! Original PTX specification:
//!
//! exit;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::exit::section_0::*;

    impl PtxUnparser for Exit {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "exit");
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
