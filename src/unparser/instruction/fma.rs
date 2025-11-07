//! Original PTX specification:
//!
//! fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
//! fma.rnd{.ftz}.f32x2      d, a, b, c;
//! fma.rnd.f64              d, a, b, c;
//! .rnd = { .rn, .rz, .rm, .rp };
//! ---------------------------------------------
//! fma.rnd{.ftz}{.sat}.f16     d, a, b, c;
//! fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
//! fma.rnd{.ftz}.relu.f16      d, a, b, c;
//! fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
//! fma.rnd{.relu}.bf16         d, a, b, c;
//! fma.rnd{.relu}.bf16x2       d, a, b, c;
//! fma.rnd.oob{.relu}.type     d, a, b, c;
//! .rnd = { .rn };
//! ---------------------------------------------
//! fma.rnd{.sat}.f32.abtype  d, a, b, c;
//! .abtype = { .f16, .bf16};
//! .rnd    = { .rn, .rz, .rm, .rp };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::fma::section_0::*;

    impl PtxUnparser for FmaRndFtzSatF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
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

    impl PtxUnparser for FmaRndFtzF32x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
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
                    push_directive(tokens, "f32x2");
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

    impl PtxUnparser for FmaRndF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
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

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::fma::section_1::*;

    impl PtxUnparser for FmaRndFtzSatF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
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
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FmaRndFtzSatF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
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
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FmaRndFtzReluF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "relu");
                    push_directive(tokens, "f16");
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

    impl PtxUnparser for FmaRndFtzReluF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    push_directive(tokens, "relu");
                    push_directive(tokens, "f16x2");
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

    impl PtxUnparser for FmaRndReluBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "bf16");
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

    impl PtxUnparser for FmaRndReluBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "bf16x2");
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

    impl PtxUnparser for FmaRndOobReluType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
                    match &self.rnd {
                            Rnd::Rn => {
                                    push_directive(tokens, "rn");
                            }
                    }
                    push_directive(tokens, "oob");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "type");
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

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::fma::section_2::*;

    impl PtxUnparser for FmaRndSatF32Abtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fma");
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
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    push_directive(tokens, "f32");
                    match &self.abtype {
                            Abtype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            Abtype::F16 => {
                                    push_directive(tokens, "f16");
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

}

