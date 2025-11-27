//! Original PTX specification:
//!
//! // Floating point (.f16 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.dtype.ctype d, a, b, c;
//! ----------------------------------------------------------------
//! // Integer (.u8/.s8 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape  =  {.m16n16k16, .m8n32k16, .m32n8k16};
//! .dtype   = {.f16, .f32};
//! .atype   = {.s8, .u8};
//! .btype   = {.s8, .u8};
//! .ctype   = {.f16, .f32};
//! ----------------------------------------------------------------
//! // Floating point format .bf16 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .atype   = {.bf16 };
//! .btype   = {.bf16};
//! ----------------------------------------------------------------
//! // Floating point format .tf32 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k8 };
//! .atype   = {.tf32 };
//! .btype   = {.tf32};
//! ----------------------------------------------------------------
//! // Floating point Double precision wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape{.rnd}.f64.f64.f64.f64 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m8n8k4 };
//! .rnd = { .rn, .rz, .rm, .rp };
//! ----------------------------------------------------------------
//! // Sub-byte (.u4/.s4 multiplicands) wmma.mma:
//! wmma.mma.sync.aligned.row.col.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .shape  = {.m8n8k32};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! ----------------------------------------------------------------
//! // Single-bit (.b1 multiplicands) wmma.mma:
//! wmma.mma.op.popc.sync.aligned.row.col.shape.s32.atype.btype.s32 d, a, b, c;
//! .shape  = {.m8n8k128};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .op     = {.xor, .and};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_0::*;

    impl PtxUnparser for WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "alayout");
                    push_directive(tokens, "blayout");
                    push_directive(tokens, "shape");
                    push_directive(tokens, "dtype");
                    push_directive(tokens, "ctype");
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

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_1::*;

    impl PtxUnparser for WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.alayout {
                            Alayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Alayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.blayout {
                            Blayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Blayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M16n16k16 => {
                                    push_directive(tokens, "m16n16k16");
                            }
                            Shape::M8n32k16 => {
                                    push_directive(tokens, "m8n32k16");
                            }
                            Shape::M32n8k16 => {
                                    push_directive(tokens, "m32n8k16");
                            }
                    }
                    push_directive(tokens, "s32");
                    match &self.atype {
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    match &self.btype {
                            Btype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Btype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    push_directive(tokens, "s32");
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
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

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_2::*;

    impl PtxUnparser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.alayout {
                            Alayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Alayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.blayout {
                            Blayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Blayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M16n16k16 => {
                                    push_directive(tokens, "m16n16k16");
                            }
                            Shape::M8n32k16 => {
                                    push_directive(tokens, "m8n32k16");
                            }
                            Shape::M32n8k16 => {
                                    push_directive(tokens, "m32n8k16");
                            }
                    }
                    push_directive(tokens, "f32");
                    match &self.atype {
                            Atype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                    }
                    match &self.btype {
                            Btype::Bf16 => {
                                    push_directive(tokens, "bf16");
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

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_3::*;

    impl PtxUnparser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.alayout {
                            Alayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Alayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.blayout {
                            Blayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Blayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M16n16k8 => {
                                    push_directive(tokens, "m16n16k8");
                            }
                    }
                    push_directive(tokens, "f32");
                    match &self.atype {
                            Atype::Tf32 => {
                                    push_directive(tokens, "tf32");
                            }
                    }
                    match &self.btype {
                            Btype::Tf32 => {
                                    push_directive(tokens, "tf32");
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

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_4::*;

    impl PtxUnparser for WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.alayout {
                            Alayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Alayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.blayout {
                            Blayout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Blayout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M8n8k4 => {
                                    push_directive(tokens, "m8n8k4");
                            }
                    }
                    if let Some(rnd_0) = self.rnd.as_ref() {
                            match rnd_0 {
                                    Rnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                                    Rnd::Rz => {
                                            push_directive(tokens, "rz");
                                    }
                                    Rnd::Rm => {
                                            push_directive(tokens, "rm");
                                    }
                                    Rnd::Rp => {
                                            push_directive(tokens, "rp");
                                    }
                            }
                    }
                    push_directive(tokens, "f64");
                    push_directive(tokens, "f64");
                    push_directive(tokens, "f64");
                    push_directive(tokens, "f64");
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

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_5::*;

    impl PtxUnparser for WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "row");
                    push_directive(tokens, "col");
                    match &self.shape {
                            Shape::M8n8k32 => {
                                    push_directive(tokens, "m8n8k32");
                            }
                    }
                    push_directive(tokens, "s32");
                    match &self.atype {
                            Atype::S4 => {
                                    push_directive(tokens, "s4");
                            }
                            Atype::U4 => {
                                    push_directive(tokens, "u4");
                            }
                    }
                    match &self.btype {
                            Btype::S4 => {
                                    push_directive(tokens, "s4");
                            }
                            Btype::U4 => {
                                    push_directive(tokens, "u4");
                            }
                    }
                    push_directive(tokens, "s32");
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
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

pub mod section_6 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_6::*;

    impl PtxUnparser for WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "mma");
                    match &self.op {
                            Op::Xor => {
                                    push_directive(tokens, "xor");
                            }
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                    }
                    push_directive(tokens, "popc");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "row");
                    push_directive(tokens, "col");
                    match &self.shape {
                            Shape::M8n8k128 => {
                                    push_directive(tokens, "m8n8k128");
                            }
                    }
                    push_directive(tokens, "s32");
                    match &self.atype {
                            Atype::B1 => {
                                    push_directive(tokens, "b1");
                            }
                    }
                    match &self.btype {
                            Btype::B1 => {
                                    push_directive(tokens, "b1");
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

}

