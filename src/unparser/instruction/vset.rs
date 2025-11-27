//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vset.atype.btype.cmp       d, a{.asel}, b{.bsel};
//! vset.atype.btype.cmp.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vset.atype.btype.cmp  d.dsel, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vset::section_0::*;

    impl PtxUnparser for VsetAtypeBtypeCmp {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vset");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
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
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for VsetAtypeBtypeCmpOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vset");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for VsetAtypeBtypeCmp1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vset");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

