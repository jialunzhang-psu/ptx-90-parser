//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; // { .hxy, where x,y are from { 0, 1, 2, 3 } };
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vset2::section_0::*;

    impl PtxUnparser for Vset2AtypeBtypeCmp {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vset2");
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
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_1) = self.asel.as_ref() {
                            match asel_1 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_2) = self.bsel.as_ref() {
                            match bsel_2 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Vset2AtypeBtypeCmpAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vset2");
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
                    if let Some(mask_3) = self.mask.as_ref() {
                            match mask_3 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_4) = self.asel.as_ref() {
                            match asel_4 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
                    if let Some(bsel_5) = self.bsel.as_ref() {
                            match bsel_5 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

