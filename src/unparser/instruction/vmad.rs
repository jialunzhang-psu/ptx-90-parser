//! Original PTX specification:
//!
//! // 32-bit scalar operation
//! vmad.dtype.atype.btype{.sat}{.scale}     d, {-}a{.asel}, {-}b{.bsel},
//! {-}c;
//! vmad.dtype.atype.btype.po{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .scale = { .shr7, .shr15 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vmad::section_0::*;

    impl PtxUnparser for VmadDtypeAtypeBtypeSatScale {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmad");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if let Some(scale_0) = self.scale.as_ref() {
                            match scale_0 {
                                    Scale::Shr7 => {
                                            push_directive(tokens, "shr7");
                                    }
                                    Scale::Shr15 => {
                                            push_directive(tokens, "shr15");
                                    }
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            if self.a_op { tokens.push(PtxToken::Minus); }
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_1) = self.asel.as_ref() {
                            match asel_1 {
                                    Asel::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Asel::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Asel::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Asel::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Asel::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Asel::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
            if self.b_op { tokens.push(PtxToken::Minus); }
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_2) = self.bsel.as_ref() {
                            match bsel_2 {
                                    Bsel::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Bsel::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Bsel::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Bsel::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Bsel::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Bsel::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
            if self.c_op { tokens.push(PtxToken::Minus); }
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for VmadDtypeAtypeBtypePoSatScale {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmad");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "po");
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if let Some(scale_3) = self.scale.as_ref() {
                            match scale_3 {
                                    Scale::Shr7 => {
                                            push_directive(tokens, "shr7");
                                    }
                                    Scale::Shr15 => {
                                            push_directive(tokens, "shr15");
                                    }
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_4) = self.asel.as_ref() {
                            match asel_4 {
                                    Asel::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Asel::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Asel::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Asel::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Asel::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Asel::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_5) = self.bsel.as_ref() {
                            match bsel_5 {
                                    Bsel::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Bsel::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Bsel::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Bsel::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Bsel::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Bsel::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

