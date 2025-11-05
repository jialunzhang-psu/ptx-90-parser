//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n }
//! n = { 0, 1, 2, 3, 4, 5, 6, 7}
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop4::section_0::*;

    impl PtxUnparser for Vop4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vop4");
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
                    self.d.unparse_tokens(tokens);
                    if let Some(mask_0) = self.mask.as_ref() {
                            match mask_0 {
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B10B2 => {
                                            push_directive(tokens, "b10.b2");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_1) = self.asel.as_ref() {
                            match asel_1 {
                                    Asel::_0 => {
                                            push_token_from_str(tokens, "0");
                                    }
                                    Asel::_1 => {
                                            push_token_from_str(tokens, "1");
                                    }
                                    Asel::_2 => {
                                            push_token_from_str(tokens, "2");
                                    }
                                    Asel::_3 => {
                                            push_token_from_str(tokens, "3");
                                    }
                                    Asel::_4 => {
                                            push_token_from_str(tokens, "4");
                                    }
                                    Asel::_5 => {
                                            push_token_from_str(tokens, "5");
                                    }
                                    Asel::_6 => {
                                            push_token_from_str(tokens, "6");
                                    }
                                    Asel::_7 => {
                                            push_token_from_str(tokens, "7");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_2) = self.bsel.as_ref() {
                            match bsel_2 {
                                    Bsel::_0 => {
                                            push_token_from_str(tokens, "0");
                                    }
                                    Bsel::_1 => {
                                            push_token_from_str(tokens, "1");
                                    }
                                    Bsel::_2 => {
                                            push_token_from_str(tokens, "2");
                                    }
                                    Bsel::_3 => {
                                            push_token_from_str(tokens, "3");
                                    }
                                    Bsel::_4 => {
                                            push_token_from_str(tokens, "4");
                                    }
                                    Bsel::_5 => {
                                            push_token_from_str(tokens, "5");
                                    }
                                    Bsel::_6 => {
                                            push_token_from_str(tokens, "6");
                                    }
                                    Bsel::_7 => {
                                            push_token_from_str(tokens, "7");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Vop4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vop4");
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
                    push_directive(tokens, "add");
                    self.d.unparse_tokens(tokens);
                    if let Some(mask_3) = self.mask.as_ref() {
                            match mask_3 {
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B10B2 => {
                                            push_directive(tokens, "b10.b2");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_4) = self.asel.as_ref() {
                            match asel_4 {
                                    Asel::_0 => {
                                            push_token_from_str(tokens, "0");
                                    }
                                    Asel::_1 => {
                                            push_token_from_str(tokens, "1");
                                    }
                                    Asel::_2 => {
                                            push_token_from_str(tokens, "2");
                                    }
                                    Asel::_3 => {
                                            push_token_from_str(tokens, "3");
                                    }
                                    Asel::_4 => {
                                            push_token_from_str(tokens, "4");
                                    }
                                    Asel::_5 => {
                                            push_token_from_str(tokens, "5");
                                    }
                                    Asel::_6 => {
                                            push_token_from_str(tokens, "6");
                                    }
                                    Asel::_7 => {
                                            push_token_from_str(tokens, "7");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_5) = self.bsel.as_ref() {
                            match bsel_5 {
                                    Bsel::_0 => {
                                            push_token_from_str(tokens, "0");
                                    }
                                    Bsel::_1 => {
                                            push_token_from_str(tokens, "1");
                                    }
                                    Bsel::_2 => {
                                            push_token_from_str(tokens, "2");
                                    }
                                    Bsel::_3 => {
                                            push_token_from_str(tokens, "3");
                                    }
                                    Bsel::_4 => {
                                            push_token_from_str(tokens, "4");
                                    }
                                    Bsel::_5 => {
                                            push_token_from_str(tokens, "5");
                                    }
                                    Bsel::_6 => {
                                            push_token_from_str(tokens, "6");
                                    }
                                    Bsel::_7 => {
                                            push_token_from_str(tokens, "7");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

