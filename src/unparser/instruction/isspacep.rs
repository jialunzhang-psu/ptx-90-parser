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
            push_opcode(tokens, "isspacep");
                    match &self.space {
                            Space::Const => {
                                    push_directive(tokens, "const");
                            }
                            Space::Global => {
                                    push_directive(tokens, "global");
                            }
                            Space::Local => {
                                    push_directive(tokens, "local");
                            }
                            Space::Shared => {
                                    push_directive(tokens, "shared");
                            }
                            Space::SharedCta => {
                                    push_directive(tokens, "shared::cta");
                            }
                            Space::SharedCluster => {
                                    push_directive(tokens, "shared::cluster");
                            }
                            Space::Param => {
                                    push_directive(tokens, "param");
                            }
                            Space::ParamEntry => {
                                    push_directive(tokens, "param::entry");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

