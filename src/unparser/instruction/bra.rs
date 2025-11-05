//! Original PTX specification:
//!
//! bra{.uni}  tgt;           // tgt is a label
//! bra{.uni}  tgt;           // unconditional branch

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::bra::section_0::*;

    impl PtxUnparser for BraUni {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "bra");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.tgt.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for BraUni1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "bra");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.tgt.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

