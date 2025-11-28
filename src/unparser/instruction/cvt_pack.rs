//! Original PTX specification:
//!
//! cvt.pack.sat.convertType.abType  d, a, b;
//! .convertType  = { .u16, .s16 };
//! .abType       = { .s32 };
//! ----------------------------------------------------------------
//! cvt.pack.sat.convertType.abType.cType  d, a, b, c;
//! .convertType  = { .u2, .s2, .u4, .s4, .u8, .s8 };
//! .abType       = { .s32 };
//! .cType        = { .b32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cvt_pack::section_0::*;

    impl PtxUnparser for CvtPackSatConverttypeAbtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cvt");
            push_directive(tokens, "pack");
            push_directive(tokens, "sat");
            match &self.converttype {
                Converttype::U16 => {
                    push_directive(tokens, "u16");
                }
                Converttype::S16 => {
                    push_directive(tokens, "s16");
                }
            }
            match &self.abtype {
                Abtype::S32 => {
                    push_directive(tokens, "s32");
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
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cvt_pack::section_1::*;

    impl PtxUnparser for CvtPackSatConverttypeAbtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cvt");
            push_directive(tokens, "pack");
            push_directive(tokens, "sat");
            match &self.converttype {
                Converttype::U2 => {
                    push_directive(tokens, "u2");
                }
                Converttype::S2 => {
                    push_directive(tokens, "s2");
                }
                Converttype::U4 => {
                    push_directive(tokens, "u4");
                }
                Converttype::S4 => {
                    push_directive(tokens, "s4");
                }
                Converttype::U8 => {
                    push_directive(tokens, "u8");
                }
                Converttype::S8 => {
                    push_directive(tokens, "s8");
                }
            }
            match &self.abtype {
                Abtype::S32 => {
                    push_directive(tokens, "s32");
                }
            }
            match &self.ctype {
                Ctype::B32 => {
                    push_directive(tokens, "b32");
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
