//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vset4.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vset4.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! defaults to .b3210
//! .asel = .bsel = { .b00, .b01, .b02, .b03, .b04, .b05, .b06, .b07,
//!                   .b10, .b11, .b12, .b13, .b14, .b15, .b16, .b17,
//!                   .b20, .b21, .b22, .b23, .b24, .b25, .b26, .b27,
//!                   .b30, .b31, .b32, .b33, .b34, .b35, .b36, .b37,
//!                   .b40, .b41, .b42, .b43, .b44, .b45, .b46, .b47,
//!                   .b50, .b51, .b52, .b53, .b54, .b55, .b56, .b57,
//!                   .b60, .b61, .b62, .b63, .b64, .b65, .b66, .b67,
//!                   .b70, .b71, .b72, .b73, .b74, .b75, .b76, .b77
//!                   } //.bxyzw, where x,y,z,w are from { 0, ..., 7 };
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vset4::section_0::*;

    impl PtxUnparser for Vset4AtypeBtypeCmp {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vset4");
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
                    match &self.cmp {
                            Cmp::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmp::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmp::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmp::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmp::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmp::Ge => {
                                    push_directive(tokens, "ge");
                            }
                    }
                    self.d.unparse_tokens(tokens);
                    if let Some(mask_0) = self.mask.as_ref() {
                            match mask_0 {
                                    Mask::B10B2 => {
                                            push_directive(tokens, "b10.b2");
                                    }
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if self.asel {
                            push_directive(tokens, "asel");
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_1) = self.bsel.as_ref() {
                            match bsel_1 {
                                    Bsel::B00 => {
                                            push_directive(tokens, "b00");
                                    }
                                    Bsel::B01 => {
                                            push_directive(tokens, "b01");
                                    }
                                    Bsel::B02 => {
                                            push_directive(tokens, "b02");
                                    }
                                    Bsel::B03 => {
                                            push_directive(tokens, "b03");
                                    }
                                    Bsel::B04 => {
                                            push_directive(tokens, "b04");
                                    }
                                    Bsel::B05 => {
                                            push_directive(tokens, "b05");
                                    }
                                    Bsel::B06 => {
                                            push_directive(tokens, "b06");
                                    }
                                    Bsel::B07 => {
                                            push_directive(tokens, "b07");
                                    }
                                    Bsel::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Bsel::B11 => {
                                            push_directive(tokens, "b11");
                                    }
                                    Bsel::B12 => {
                                            push_directive(tokens, "b12");
                                    }
                                    Bsel::B13 => {
                                            push_directive(tokens, "b13");
                                    }
                                    Bsel::B14 => {
                                            push_directive(tokens, "b14");
                                    }
                                    Bsel::B15 => {
                                            push_directive(tokens, "b15");
                                    }
                                    Bsel::B16 => {
                                            push_directive(tokens, "b16");
                                    }
                                    Bsel::B17 => {
                                            push_directive(tokens, "b17");
                                    }
                                    Bsel::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Bsel::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Bsel::B22 => {
                                            push_directive(tokens, "b22");
                                    }
                                    Bsel::B23 => {
                                            push_directive(tokens, "b23");
                                    }
                                    Bsel::B24 => {
                                            push_directive(tokens, "b24");
                                    }
                                    Bsel::B25 => {
                                            push_directive(tokens, "b25");
                                    }
                                    Bsel::B26 => {
                                            push_directive(tokens, "b26");
                                    }
                                    Bsel::B27 => {
                                            push_directive(tokens, "b27");
                                    }
                                    Bsel::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Bsel::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Bsel::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Bsel::B33 => {
                                            push_directive(tokens, "b33");
                                    }
                                    Bsel::B34 => {
                                            push_directive(tokens, "b34");
                                    }
                                    Bsel::B35 => {
                                            push_directive(tokens, "b35");
                                    }
                                    Bsel::B36 => {
                                            push_directive(tokens, "b36");
                                    }
                                    Bsel::B37 => {
                                            push_directive(tokens, "b37");
                                    }
                                    Bsel::B40 => {
                                            push_directive(tokens, "b40");
                                    }
                                    Bsel::B41 => {
                                            push_directive(tokens, "b41");
                                    }
                                    Bsel::B42 => {
                                            push_directive(tokens, "b42");
                                    }
                                    Bsel::B43 => {
                                            push_directive(tokens, "b43");
                                    }
                                    Bsel::B44 => {
                                            push_directive(tokens, "b44");
                                    }
                                    Bsel::B45 => {
                                            push_directive(tokens, "b45");
                                    }
                                    Bsel::B46 => {
                                            push_directive(tokens, "b46");
                                    }
                                    Bsel::B47 => {
                                            push_directive(tokens, "b47");
                                    }
                                    Bsel::B50 => {
                                            push_directive(tokens, "b50");
                                    }
                                    Bsel::B51 => {
                                            push_directive(tokens, "b51");
                                    }
                                    Bsel::B52 => {
                                            push_directive(tokens, "b52");
                                    }
                                    Bsel::B53 => {
                                            push_directive(tokens, "b53");
                                    }
                                    Bsel::B54 => {
                                            push_directive(tokens, "b54");
                                    }
                                    Bsel::B55 => {
                                            push_directive(tokens, "b55");
                                    }
                                    Bsel::B56 => {
                                            push_directive(tokens, "b56");
                                    }
                                    Bsel::B57 => {
                                            push_directive(tokens, "b57");
                                    }
                                    Bsel::B60 => {
                                            push_directive(tokens, "b60");
                                    }
                                    Bsel::B61 => {
                                            push_directive(tokens, "b61");
                                    }
                                    Bsel::B62 => {
                                            push_directive(tokens, "b62");
                                    }
                                    Bsel::B63 => {
                                            push_directive(tokens, "b63");
                                    }
                                    Bsel::B64 => {
                                            push_directive(tokens, "b64");
                                    }
                                    Bsel::B65 => {
                                            push_directive(tokens, "b65");
                                    }
                                    Bsel::B66 => {
                                            push_directive(tokens, "b66");
                                    }
                                    Bsel::B67 => {
                                            push_directive(tokens, "b67");
                                    }
                                    Bsel::B70 => {
                                            push_directive(tokens, "b70");
                                    }
                                    Bsel::B71 => {
                                            push_directive(tokens, "b71");
                                    }
                                    Bsel::B72 => {
                                            push_directive(tokens, "b72");
                                    }
                                    Bsel::B73 => {
                                            push_directive(tokens, "b73");
                                    }
                                    Bsel::B74 => {
                                            push_directive(tokens, "b74");
                                    }
                                    Bsel::B75 => {
                                            push_directive(tokens, "b75");
                                    }
                                    Bsel::B76 => {
                                            push_directive(tokens, "b76");
                                    }
                                    Bsel::B77 => {
                                            push_directive(tokens, "b77");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Vset4AtypeBtypeCmpAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vset4");
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
                    match &self.cmp {
                            Cmp::Eq => {
                                    push_directive(tokens, "eq");
                            }
                            Cmp::Ne => {
                                    push_directive(tokens, "ne");
                            }
                            Cmp::Lt => {
                                    push_directive(tokens, "lt");
                            }
                            Cmp::Le => {
                                    push_directive(tokens, "le");
                            }
                            Cmp::Gt => {
                                    push_directive(tokens, "gt");
                            }
                            Cmp::Ge => {
                                    push_directive(tokens, "ge");
                            }
                    }
                    push_directive(tokens, "add");
                    self.d.unparse_tokens(tokens);
                    if let Some(mask_2) = self.mask.as_ref() {
                            match mask_2 {
                                    Mask::B10B2 => {
                                            push_directive(tokens, "b10.b2");
                                    }
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if self.asel {
                            push_directive(tokens, "asel");
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_3) = self.bsel.as_ref() {
                            match bsel_3 {
                                    Bsel::B00 => {
                                            push_directive(tokens, "b00");
                                    }
                                    Bsel::B01 => {
                                            push_directive(tokens, "b01");
                                    }
                                    Bsel::B02 => {
                                            push_directive(tokens, "b02");
                                    }
                                    Bsel::B03 => {
                                            push_directive(tokens, "b03");
                                    }
                                    Bsel::B04 => {
                                            push_directive(tokens, "b04");
                                    }
                                    Bsel::B05 => {
                                            push_directive(tokens, "b05");
                                    }
                                    Bsel::B06 => {
                                            push_directive(tokens, "b06");
                                    }
                                    Bsel::B07 => {
                                            push_directive(tokens, "b07");
                                    }
                                    Bsel::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Bsel::B11 => {
                                            push_directive(tokens, "b11");
                                    }
                                    Bsel::B12 => {
                                            push_directive(tokens, "b12");
                                    }
                                    Bsel::B13 => {
                                            push_directive(tokens, "b13");
                                    }
                                    Bsel::B14 => {
                                            push_directive(tokens, "b14");
                                    }
                                    Bsel::B15 => {
                                            push_directive(tokens, "b15");
                                    }
                                    Bsel::B16 => {
                                            push_directive(tokens, "b16");
                                    }
                                    Bsel::B17 => {
                                            push_directive(tokens, "b17");
                                    }
                                    Bsel::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Bsel::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Bsel::B22 => {
                                            push_directive(tokens, "b22");
                                    }
                                    Bsel::B23 => {
                                            push_directive(tokens, "b23");
                                    }
                                    Bsel::B24 => {
                                            push_directive(tokens, "b24");
                                    }
                                    Bsel::B25 => {
                                            push_directive(tokens, "b25");
                                    }
                                    Bsel::B26 => {
                                            push_directive(tokens, "b26");
                                    }
                                    Bsel::B27 => {
                                            push_directive(tokens, "b27");
                                    }
                                    Bsel::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Bsel::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Bsel::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Bsel::B33 => {
                                            push_directive(tokens, "b33");
                                    }
                                    Bsel::B34 => {
                                            push_directive(tokens, "b34");
                                    }
                                    Bsel::B35 => {
                                            push_directive(tokens, "b35");
                                    }
                                    Bsel::B36 => {
                                            push_directive(tokens, "b36");
                                    }
                                    Bsel::B37 => {
                                            push_directive(tokens, "b37");
                                    }
                                    Bsel::B40 => {
                                            push_directive(tokens, "b40");
                                    }
                                    Bsel::B41 => {
                                            push_directive(tokens, "b41");
                                    }
                                    Bsel::B42 => {
                                            push_directive(tokens, "b42");
                                    }
                                    Bsel::B43 => {
                                            push_directive(tokens, "b43");
                                    }
                                    Bsel::B44 => {
                                            push_directive(tokens, "b44");
                                    }
                                    Bsel::B45 => {
                                            push_directive(tokens, "b45");
                                    }
                                    Bsel::B46 => {
                                            push_directive(tokens, "b46");
                                    }
                                    Bsel::B47 => {
                                            push_directive(tokens, "b47");
                                    }
                                    Bsel::B50 => {
                                            push_directive(tokens, "b50");
                                    }
                                    Bsel::B51 => {
                                            push_directive(tokens, "b51");
                                    }
                                    Bsel::B52 => {
                                            push_directive(tokens, "b52");
                                    }
                                    Bsel::B53 => {
                                            push_directive(tokens, "b53");
                                    }
                                    Bsel::B54 => {
                                            push_directive(tokens, "b54");
                                    }
                                    Bsel::B55 => {
                                            push_directive(tokens, "b55");
                                    }
                                    Bsel::B56 => {
                                            push_directive(tokens, "b56");
                                    }
                                    Bsel::B57 => {
                                            push_directive(tokens, "b57");
                                    }
                                    Bsel::B60 => {
                                            push_directive(tokens, "b60");
                                    }
                                    Bsel::B61 => {
                                            push_directive(tokens, "b61");
                                    }
                                    Bsel::B62 => {
                                            push_directive(tokens, "b62");
                                    }
                                    Bsel::B63 => {
                                            push_directive(tokens, "b63");
                                    }
                                    Bsel::B64 => {
                                            push_directive(tokens, "b64");
                                    }
                                    Bsel::B65 => {
                                            push_directive(tokens, "b65");
                                    }
                                    Bsel::B66 => {
                                            push_directive(tokens, "b66");
                                    }
                                    Bsel::B67 => {
                                            push_directive(tokens, "b67");
                                    }
                                    Bsel::B70 => {
                                            push_directive(tokens, "b70");
                                    }
                                    Bsel::B71 => {
                                            push_directive(tokens, "b71");
                                    }
                                    Bsel::B72 => {
                                            push_directive(tokens, "b72");
                                    }
                                    Bsel::B73 => {
                                            push_directive(tokens, "b73");
                                    }
                                    Bsel::B74 => {
                                            push_directive(tokens, "b74");
                                    }
                                    Bsel::B75 => {
                                            push_directive(tokens, "b75");
                                    }
                                    Bsel::B76 => {
                                            push_directive(tokens, "b76");
                                    }
                                    Bsel::B77 => {
                                            push_directive(tokens, "b77");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

