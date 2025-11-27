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
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cvta");
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
                    match &self.size {
                            Size::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Size::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for CvtaToSpaceSize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cvta");
                    push_directive(tokens, "to");
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
                    match &self.size {
                            Size::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Size::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.p.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

