//! Original PTX specification:
//!
//! mad.mode.type  d, a, b, c;
//! mad.hi.sat.s32 d, a, b, c;
//! .mode = { .hi, .lo, .wide };
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! 
//! mad{.ftz}{.sat}.f32      d, a, b, c;    // .target sm_1x
//! mad.rnd{.ftz}{.sat}.f32  d, a, b, c;    // .target sm_20
//! mad.rnd.f64              d, a, b, c;    // .target sm_13 and higher
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mad::section_0::*;

    impl PtxUnparser for MadModeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad");
                    match &self.mode {
                            Mode::Hi => {
                                    push_directive(tokens, "hi");
                            }
                            Mode::Lo => {
                                    push_directive(tokens, "lo");
                            }
                            Mode::Wide => {
                                    push_directive(tokens, "wide");
                            }
                    }
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
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MadHiSatS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad");
                    push_directive(tokens, "hi");
                    push_directive(tokens, "sat");
                    push_directive(tokens, "s32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MadFtzSatF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MadRndFtzSatF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad");
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
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MadRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mad");
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
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

