//! Original PTX specification:
//!
//! div.type  d, a, b;
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! 
//! div.approx{.ftz}.f32  d, a, b;  // fast, approximate divide
//! div.full{.ftz}.f32    d, a, b;  // full-range approximate divide
//! div.rnd{.ftz}.f32     d, a, b;  // IEEE 754 compliant rounding
//! div.rnd.f64           d, a, b;  // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::div::section_0::*;

    impl PtxUnparser for DivType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "div");
                    match &self.type_ {
                            Type::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for DivApproxFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "div");
                    push_directive(tokens, "approx");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for DivFullFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "div");
                    push_directive(tokens, "full");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for DivRndFtzF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "div");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Rnd::Rz => {
                                    push_directive(tokens, "rz");
                            }
                            Rnd::Rm => {
                                    push_directive(tokens, "rm");
                            }
                            Rnd::Rp => {
                                    push_directive(tokens, "rp");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for DivRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "div");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Rnd::Rz => {
                                    push_directive(tokens, "rz");
                            }
                            Rnd::Rm => {
                                    push_directive(tokens, "rm");
                            }
                            Rnd::Rp => {
                                    push_directive(tokens, "rp");
                            }
                    }
                    push_directive(tokens, "f64");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

