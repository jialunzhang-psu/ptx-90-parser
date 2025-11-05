//! Original PTX specification:
//!
//! max.atype         d, a, b;
//! max{.relu}.btype  d, a, b;
//! .atype = { .u16, .u32, .u64,
//! .u16x2, .s16, .s64 };
//! .btype = { .s16x2, .s32 };
//! 
//! max{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
//! max{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
//! max.f64                            d, a, b;
//! 
//! max{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
//! max{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
//! max{.NaN}{.xorsign.abs}.bf16           d, a, b;
//! max{.NaN}{.xorsign.abs}.bf16x2         d, a, b;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::max::section_0::*;

    impl PtxUnparser for MaxAtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    match &self.atype {
                            Atype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Atype::U16x2 => {
                                    push_directive(tokens, "u16x2");
                            }
                            Atype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Atype::S64 => {
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

    impl PtxUnparser for MaxReluBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    match &self.btype {
                            Btype::S16x2 => {
                                    push_directive(tokens, "s16x2");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
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

    impl PtxUnparser for MaxFtzNanXorsignAbsF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.xorsign_abs {
                            push_directive(tokens, "xorsign.abs");
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

    impl PtxUnparser for MaxFtzNanAbsF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.abs {
                            push_directive(tokens, "abs");
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

    impl PtxUnparser for MaxF64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    push_directive(tokens, "f64");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MaxFtzNanXorsignAbsF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.xorsign_abs {
                            push_directive(tokens, "xorsign.abs");
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

    impl PtxUnparser for MaxFtzNanXorsignAbsF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.xorsign_abs {
                            push_directive(tokens, "xorsign.abs");
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

    impl PtxUnparser for MaxNanXorsignAbsBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.xorsign_abs {
                            push_directive(tokens, "xorsign.abs");
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

    impl PtxUnparser for MaxNanXorsignAbsBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "max");
                    if self.nan {
                            push_directive(tokens, "NaN");
                    }
                    if self.xorsign_abs {
                            push_directive(tokens, "xorsign.abs");
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

