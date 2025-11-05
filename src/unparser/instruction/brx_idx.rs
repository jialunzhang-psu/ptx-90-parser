//! Original PTX specification:
//!
//! brx.idx{.uni} index, tlist;
//! brx.idx{.uni} index, tlist;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::brx_idx::section_0::*;

    impl PtxUnparser for BrxIdxUni {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "brx");
                    push_directive(tokens, "idx");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.index.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.tlist.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for BrxIdxUni1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "brx");
                    push_directive(tokens, "idx");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.index.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.tlist.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

