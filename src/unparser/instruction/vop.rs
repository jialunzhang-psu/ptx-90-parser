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

    impl PtxUnparser for VaddDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vadd");
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

    impl PtxUnparser for VsubDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vsub");
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
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for VabsdiffDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vabsdiff");
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
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for VminDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmin");
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
                    if let Some(asel_6) = self.asel.as_ref() {
                            match asel_6 {
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
                    if let Some(bsel_7) = self.bsel.as_ref() {
                            match bsel_7 {
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

    impl PtxUnparser for VmaxDtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmax");
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
                    if let Some(asel_8) = self.asel.as_ref() {
                            match asel_8 {
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
                    if let Some(bsel_9) = self.bsel.as_ref() {
                            match bsel_9 {
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

    impl PtxUnparser for VaddDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vadd");
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
                    if let Some(asel_10) = self.asel.as_ref() {
                            match asel_10 {
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
                    if let Some(bsel_11) = self.bsel.as_ref() {
                            match bsel_11 {
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

    impl PtxUnparser for VsubDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vsub");
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
                    if let Some(asel_12) = self.asel.as_ref() {
                            match asel_12 {
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
                    if let Some(bsel_13) = self.bsel.as_ref() {
                            match bsel_13 {
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

    impl PtxUnparser for VabsdiffDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vabsdiff");
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
                    if let Some(asel_14) = self.asel.as_ref() {
                            match asel_14 {
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
                    if let Some(bsel_15) = self.bsel.as_ref() {
                            match bsel_15 {
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

    impl PtxUnparser for VminDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmin");
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
                    if let Some(asel_16) = self.asel.as_ref() {
                            match asel_16 {
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
                    if let Some(bsel_17) = self.bsel.as_ref() {
                            match bsel_17 {
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

    impl PtxUnparser for VmaxDtypeAtypeBtypeSatOp2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmax");
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
                    if let Some(asel_18) = self.asel.as_ref() {
                            match asel_18 {
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
                    if let Some(bsel_19) = self.bsel.as_ref() {
                            match bsel_19 {
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

    impl PtxUnparser for VaddDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vadd");
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
                    if let Some(asel_20) = self.asel.as_ref() {
                            match asel_20 {
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
                    if let Some(bsel_21) = self.bsel.as_ref() {
                            match bsel_21 {
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

    impl PtxUnparser for VsubDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vsub");
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
                    if let Some(asel_22) = self.asel.as_ref() {
                            match asel_22 {
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
                    if let Some(bsel_23) = self.bsel.as_ref() {
                            match bsel_23 {
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

    impl PtxUnparser for VabsdiffDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vabsdiff");
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
                    if let Some(asel_24) = self.asel.as_ref() {
                            match asel_24 {
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
                    if let Some(bsel_25) = self.bsel.as_ref() {
                            match bsel_25 {
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

    impl PtxUnparser for VminDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmin");
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
                    if let Some(asel_26) = self.asel.as_ref() {
                            match asel_26 {
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
                    if let Some(bsel_27) = self.bsel.as_ref() {
                            match bsel_27 {
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

    impl PtxUnparser for VmaxDtypeAtypeBtypeSat1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "vmax");
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
                    if let Some(asel_28) = self.asel.as_ref() {
                            match asel_28 {
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
                    if let Some(bsel_29) = self.bsel.as_ref() {
                            match bsel_29 {
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

