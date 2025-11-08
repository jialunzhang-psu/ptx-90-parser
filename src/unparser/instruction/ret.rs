//! Original PTX specification:
//!
//! ret{.uni};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::ret::section_0::*;

    impl PtxUnparser for RetUni {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "ret");
            if self.uni {
                push_directive(tokens, "uni");
            }
            tokens.push(PtxToken::Semicolon);
        }
    }
}
