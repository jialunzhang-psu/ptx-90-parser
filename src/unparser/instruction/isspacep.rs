//! Original PTX specification:
//!
//! isspacep.space  p, a;    // result is .pred
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::isspacep::section_0::*;

    impl PtxUnparser for IsspacepSpace {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "isspacep");
            match &self.space {
                Space::SharedCluster => {
                    push_directive(tokens, "shared::cluster");
                }
                Space::ParamEntry => {
                    push_directive(tokens, "param::entry");
                }
                Space::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
                Space::Global => {
                    push_directive(tokens, "global");
                }
                Space::Shared => {
                    push_directive(tokens, "shared");
                }
                Space::Const => {
                    push_directive(tokens, "const");
                }
                Space::Local => {
                    push_directive(tokens, "local");
                }
                Space::Param => {
                    push_directive(tokens, "param");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
