//! Original PTX specification:
//!
//! discard{.global}.level  [a], size;
//! .level = { .L2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::discard::section_0::*;

    impl PtxUnparser for DiscardGlobalLevel {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "discard");
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.level {
                            Level::L2 => {
                                    push_directive(tokens, "L2");
                            }
                    }
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

