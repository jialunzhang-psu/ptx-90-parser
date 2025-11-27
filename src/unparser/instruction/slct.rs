//! Original PTX specification:
//!
//! slct.dtype.s32        d, a, b, c;
//! slct{.ftz}.dtype.f32  d, a, b, c;
//! .dtype = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::slct::section_0::*;

    impl PtxUnparser for SlctDtypeS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "slct");
                    match &self.dtype {
                            Dtype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Dtype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Dtype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Dtype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Dtype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    push_directive(tokens, "s32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for SlctFtzDtypeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "slct");
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    match &self.dtype {
                            Dtype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Dtype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Dtype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Dtype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Dtype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Dtype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Dtype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    push_directive(tokens, "f32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

