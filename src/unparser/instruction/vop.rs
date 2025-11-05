//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vadd, vsub, vabsdiff, vmin, vmax };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop::section_0::*;

    impl PtxUnparser for VopDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vop");
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
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_0) = self.asel.as_ref() {
                            match asel_0 {
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
                    if let Some(bsel_1) = self.bsel.as_ref() {
                            match bsel_1 {
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
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for VopDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vop");
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
                    match &self.op2 {
                            Op2::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op2::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op2::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
                    if let Some(asel_2) = self.asel.as_ref() {
                            match asel_2 {
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
                    if let Some(bsel_3) = self.bsel.as_ref() {
                            match bsel_3 {
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

    impl PtxUnparser for VopDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vop");
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
            tokens.push(PtxToken::Comma);
                    match &self.dsel {
                        Dsel::B0 => {
                                    push_directive(tokens, "b0");
                        }
                        Dsel::B1 => {
                                    push_directive(tokens, "b1");
                        }
                        Dsel::B2 => {
                                    push_directive(tokens, "b2");
                        }
                        Dsel::B3 => {
                                    push_directive(tokens, "b3");
                        }
                        Dsel::H0 => {
                                    push_directive(tokens, "h0");
                        }
                        Dsel::H1 => {
                                    push_directive(tokens, "h1");
                        }
                    }
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

