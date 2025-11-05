//! Original PTX specification:
//!
//! mul.mode.type  d, a, b;
//! .mode = { .hi, .lo, .wide };
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! mul{.rnd}{.ftz}.f32x2      d, a, b;
//! mul{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! mul{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! mul{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! mul{.rnd}.bf16   d, a, b;
//! mul{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mul::section_0::*;

    impl PtxUnparser for MulModeType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
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
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mul::section_1::*;

    impl PtxUnparser for MulRndFtzSatF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_0) = self.rnd.as_ref() {
                            match rnd_0 {
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
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MulRndFtzF32x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_1) = self.rnd.as_ref() {
                            match rnd_1 {
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
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "f32x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MulRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_2) = self.rnd.as_ref() {
                            match rnd_2 {
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

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::mul::section_2::*;

    impl PtxUnparser for MulRndFtzSatF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_3) = self.rnd.as_ref() {
                            match rnd_3 {
                                    Rnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    push_directive(tokens, "f16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MulRndFtzSatF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_4) = self.rnd.as_ref() {
                            match rnd_4 {
                                    Rnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    push_directive(tokens, "f16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MulRndBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_5) = self.rnd.as_ref() {
                            match rnd_5 {
                                    Rnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MulRndBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mul");
                    if let Some(rnd_6) = self.rnd.as_ref() {
                            match rnd_6 {
                                    Rnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

