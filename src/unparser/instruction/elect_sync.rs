//! Original PTX specification:
//!
//! elect.sync d|p, membermask;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::elect_sync::section_0::*;

    impl PtxUnparser for ElectSync {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "elect");
                    push_directive(tokens, "sync");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.membermask.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

