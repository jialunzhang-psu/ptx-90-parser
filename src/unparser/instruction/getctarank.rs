//! Original PTX specification:
//!
//! getctarank{.space}.type d, a;
//! // Get cta rank from source shared memory address in register a.
//! getctarank.shared::cluster.type d, a;
//! // // Get cta rank from shared memory variable.
//! // getctarank.shared::cluster.type d, var;
//! // // Get cta rank from shared memory variable+offset.
//! // getctarank.shared::cluster.type d, var + imm;
//! // Get cta rank from generic address of shared memory variable in register a.
//! getctarank.type d, a;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::getctarank::section_0::*;

    impl PtxUnparser for GetctarankSpaceType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "getctarank");
                    if let Some(space_0) = self.space.as_ref() {
                            match space_0 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for GetctarankSharedClusterType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "getctarank");
                    push_directive(tokens, "shared::cluster");
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for GetctarankType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "getctarank");
                    match &self.type_ {
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

