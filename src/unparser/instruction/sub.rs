//! Original PTX specification:
//!
//! sub.type       d, a, b;
//! sub{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! sub{.rnd}{.ftz}.f32x2      d, a, b;
//! sub{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! sub{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! sub{.rnd}.bf16   d, a, b;
//! sub{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! sub{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sub::section_0::*;

    impl PtxUnparser for SubType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubSatS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
            if self.sat {
                push_directive(tokens, "sat");
            }
            push_directive(tokens, "s32");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::sub::section_1::*;

    impl PtxUnparser for SubRndFtzSatF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubRndFtzF32x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::sub::section_2::*;

    impl PtxUnparser for SubRndFtzSatF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubRndFtzSatF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
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
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubRndBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
            if let Some(rnd_5) = self.rnd.as_ref() {
                match rnd_5 {
                    Rnd::Rn => {
                        push_directive(tokens, "rn");
                    }
                }
            }
            push_directive(tokens, "bf16");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }

    impl PtxUnparser for SubRndBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
            if let Some(rnd_6) = self.rnd.as_ref() {
                match rnd_6 {
                    Rnd::Rn => {
                        push_directive(tokens, "rn");
                    }
                }
            }
            push_directive(tokens, "bf16x2");
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::sub::section_3::*;

    impl PtxUnparser for SubRndSatF32Atype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "sub");
            if let Some(rnd_7) = self.rnd.as_ref() {
                match rnd_7 {
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
            if self.sat {
                push_directive(tokens, "sat");
            }
            push_directive(tokens, "f32");
            match &self.atype {
                Atype::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Atype::F16 => {
                    push_directive(tokens, "f16");
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
