//! Original PTX specification:
//!
//! nanosleep.u32 t;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::nanosleep::section_0::*;

    impl PtxUnparser for NanosleepU32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "nanosleep");
            push_directive(tokens, "u32");
            self.t.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
