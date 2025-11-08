//! Original PTX specification:
//!
//! stacksave.type  d;
//! .type = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::stacksave::section_0::*;

    impl PtxUnparser for StacksaveType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "stacksave");
            match &self.type_ {
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
