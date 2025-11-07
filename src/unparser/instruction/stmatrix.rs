//! Original PTX specification:
//!
//! stmatrix.sync.aligned.shape.num{.trans}{.ss}.type [p], r;
//! .shape  = {.m8n8, .m16n8};
//! .num    = {.x1, .x2, .x4};
//! .ss     = {.shared, .shared::cta};
//! .type   = {.b16, .b8};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::stmatrix::section_0::*;

    impl PtxUnparser for StmatrixSyncAlignedShapeNumTransSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "stmatrix");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.shape {
                            Shape::M16n8 => {
                                    push_directive(tokens, "m16n8");
                            }
                            Shape::M8n8 => {
                                    push_directive(tokens, "m8n8");
                            }
                    }
                    match &self.num {
                            Num::X1 => {
                                    push_directive(tokens, "x1");
                            }
                            Num::X2 => {
                                    push_directive(tokens, "x2");
                            }
                            Num::X4 => {
                                    push_directive(tokens, "x4");
                            }
                    }
                    if self.trans {
                            push_directive(tokens, "trans");
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Type::B8 => {
                                    push_directive(tokens, "b8");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

