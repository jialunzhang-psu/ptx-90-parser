//! Original PTX specification:
//!
//! brkpt;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::brkpt::section_0::*;

    impl PtxUnparser for Brkpt {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "brkpt");
            tokens.push(PtxToken::Semicolon);
        }
    }

}

