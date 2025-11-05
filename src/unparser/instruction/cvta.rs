//! Original PTX specification:
//!
//! // convert const, global, local, or shared address to generic address
//! cvta.space.size  p, a;        // source address in register a
//! // cvta.space.size  p, var;      // get generic address of var
//! // cvta.space.size  p, var+imm;  // generic address of var+offset
//! // convert generic address to const, global, local, or shared address
//! cvta.to.space.size  p, a;
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };
//! .size  = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cvta::section_0::*;

    impl PtxUnparser for CvtaSpaceSize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvta");
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
                    match &self.size {
                            Size::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Size::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtaToSpaceSize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvta");
                    push_directive(tokens, "to");
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
                    match &self.size {
                            Size::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Size::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

