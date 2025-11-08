//! Original PTX specification:
//!
//! trap;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::trap::section_0::*;

    impl PtxUnparser for Trap {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "trap");
            tokens.push(PtxToken::Semicolon);
        }
    }
}
