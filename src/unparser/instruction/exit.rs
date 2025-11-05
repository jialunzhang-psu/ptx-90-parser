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
            push_opcode(tokens, "exit");
            tokens.push(PtxToken::Semicolon);
        }
    }

}

