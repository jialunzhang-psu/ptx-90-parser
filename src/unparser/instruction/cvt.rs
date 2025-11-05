//! Original PTX specification:
//!
//! cvt{.irnd}{.ftz}{.sat}.dtype.atype         d, a;  // integer rounding
//! cvt{.frnd}{.ftz}{.sat}.dtype.atype         d, a;  // fp rounding
//! cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
//! cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
//! cvt.rs{.relu}{.satfinite}.f16x2.f32        d, a, b, rbits;
//! cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
//! cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
//! cvt.rs{.relu}{.satfinite}.bf16x2.f32       d, a, b, rbits;
//! cvt.rna{.satfinite}.tf32.f32               d, a;
//! cvt.frnd2{.satfinite}{.relu}.tf32.f32      d, a;
//! cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
//! cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
//! cvt.rn{.relu}.f16x2.f8x2type              d, a;
//! cvt.rs{.relu}.satfinite.f8x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f4x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f4x2type               d, a;
//! cvt.rs{.relu}.satfinite.f4x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.rn.satfinite{.relu}.f6x2type.f32       d, a, b;
//! cvt.rn{.relu}.f16x2.f6x2type               d, a;
//! cvt.rs{.relu}.satfinite.f6x4type.f32       d, {a, b, e, f}, rbits;
//! cvt.frnd3{.satfinite}.ue8m0x2.f32          d, a, b;
//! cvt.frnd3{.satfinite}.ue8m0x2.bf16x2       d, a;
//! cvt.rn.bf16x2.ue8m0x2                      d, a;
//! .irnd   = { .rni, .rzi, .rmi, .rpi };
//! .frnd   = { .rn,  .rz,  .rm,  .rp  };
//! .frnd2  = { .rn,  .rz };
//! .frnd3  = { .rz,  .rp };
//! .dtype = .atype = { .u8,   .u16, .u32, .u64,
//! .s8,   .s16, .s32, .s64,
//! .bf16, .f16, .f32, .f64 };
//! .f8x2type = { .e4m3x2, .e5m2x2 };
//! .f4x2type = { .e2m1x2 };
//! .f6x2type = { .e2m3x2, .e3m2x2 };
//! .f4x4type = { .e2m1x4 };
//! .f8x4type = { .e4m3x4, .e5m2x4 };
//! .f6x4type = { .e2m3x4, .e3m2x4 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cvt::section_0::*;

    impl PtxUnparser for CvtIrndFtzSatDtypeAtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    if let Some(irnd_0) = self.irnd.as_ref() {
                            match irnd_0 {
                                    Irnd::Rni => {
                                            push_directive(tokens, "rni");
                                    }
                                    Irnd::Rzi => {
                                            push_directive(tokens, "rzi");
                                    }
                                    Irnd::Rmi => {
                                            push_directive(tokens, "rmi");
                                    }
                                    Irnd::Rpi => {
                                            push_directive(tokens, "rpi");
                                    }
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    match &self.dtype {
                            Dtype::U8 => {
                                    push_directive(tokens, "u8");
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
                            Dtype::S8 => {
                                    push_directive(tokens, "s8");
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
                            Dtype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Dtype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    match &self.atype {
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Atype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Atype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Atype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            Atype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Atype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Atype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrndFtzSatDtypeAtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    if let Some(frnd_1) = self.frnd.as_ref() {
                            match frnd_1 {
                                    Frnd::Rn => {
                                            push_directive(tokens, "rn");
                                    }
                                    Frnd::Rz => {
                                            push_directive(tokens, "rz");
                                    }
                                    Frnd::Rm => {
                                            push_directive(tokens, "rm");
                                    }
                                    Frnd::Rp => {
                                            push_directive(tokens, "rp");
                                    }
                            }
                    }
                    if self.ftz {
                            push_directive(tokens, "ftz");
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    match &self.dtype {
                            Dtype::U8 => {
                                    push_directive(tokens, "u8");
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
                            Dtype::S8 => {
                                    push_directive(tokens, "s8");
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
                            Dtype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Dtype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    match &self.atype {
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                            Atype::U16 => {
                                    push_directive(tokens, "u16");
                            }
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::S16 => {
                                    push_directive(tokens, "s16");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Atype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Atype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            Atype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Atype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Atype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd2ReluSatfiniteF16F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd2 {
                            Frnd2::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Frnd2::Rz => {
                                    push_directive(tokens, "rz");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "f16");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd2ReluSatfiniteF16x2F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd2 {
                            Frnd2::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Frnd2::Rz => {
                                    push_directive(tokens, "rz");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "f16x2");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRsReluSatfiniteF16x2F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rs");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "f16x2");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.rbits.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd2ReluSatfiniteBf16F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd2 {
                            Frnd2::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Frnd2::Rz => {
                                    push_directive(tokens, "rz");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "bf16");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd2ReluSatfiniteBf16x2F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd2 {
                            Frnd2::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Frnd2::Rz => {
                                    push_directive(tokens, "rz");
                            }
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "bf16x2");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRsReluSatfiniteBf16x2F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rs");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "bf16x2");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.rbits.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnaSatfiniteTf32F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rna");
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "tf32");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd2SatfiniteReluTf32F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd2 {
                            Frnd2::Rn => {
                                    push_directive(tokens, "rn");
                            }
                            Frnd2::Rz => {
                                    push_directive(tokens, "rz");
                            }
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "tf32");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnSatfiniteReluF8x2typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    push_directive(tokens, "satfinite");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    match &self.f8x2type {
                            F8x2type::E4m3x2 => {
                                    push_directive(tokens, "e4m3x2");
                            }
                            F8x2type::E5m2x2 => {
                                    push_directive(tokens, "e5m2x2");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnSatfiniteReluF8x2typeF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    push_directive(tokens, "satfinite");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    match &self.f8x2type {
                            F8x2type::E4m3x2 => {
                                    push_directive(tokens, "e4m3x2");
                            }
                            F8x2type::E5m2x2 => {
                                    push_directive(tokens, "e5m2x2");
                            }
                    }
                    push_directive(tokens, "f16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnReluF16x2F8x2type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "f16x2");
                    match &self.f8x2type {
                            F8x2type::E4m3x2 => {
                                    push_directive(tokens, "e4m3x2");
                            }
                            F8x2type::E5m2x2 => {
                                    push_directive(tokens, "e5m2x2");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRsReluSatfiniteF8x4typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rs");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "satfinite");
                    match &self.f8x4type {
                            F8x4type::E4m3x4 => {
                                    push_directive(tokens, "e4m3x4");
                            }
                            F8x4type::E5m2x4 => {
                                    push_directive(tokens, "e5m2x4");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBrace);
                    let &( ref group_2_0, ref group_2_1, ref group_2_2, ref group_2_3) = &self.a;
                    group_2_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_2_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_2_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_2_3.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBrace);
            tokens.push(PtxToken::Comma);
                    self.rbits.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnSatfiniteReluF4x2typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    push_directive(tokens, "satfinite");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    match &self.f4x2type {
                            F4x2type::E2m1x2 => {
                                    push_directive(tokens, "e2m1x2");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnReluF16x2F4x2type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "f16x2");
                    match &self.f4x2type {
                            F4x2type::E2m1x2 => {
                                    push_directive(tokens, "e2m1x2");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRsReluSatfiniteF4x4typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rs");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "satfinite");
                    match &self.f4x4type {
                            F4x4type::E2m1x4 => {
                                    push_directive(tokens, "e2m1x4");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBrace);
                    let &( ref group_3_0, ref group_3_1, ref group_3_2, ref group_3_3) = &self.a;
                    group_3_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_3.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBrace);
            tokens.push(PtxToken::Comma);
                    self.rbits.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnSatfiniteReluF6x2typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    push_directive(tokens, "satfinite");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    match &self.f6x2type {
                            F6x2type::E2m3x2 => {
                                    push_directive(tokens, "e2m3x2");
                            }
                            F6x2type::E3m2x2 => {
                                    push_directive(tokens, "e3m2x2");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnReluF16x2F6x2type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "f16x2");
                    match &self.f6x2type {
                            F6x2type::E2m3x2 => {
                                    push_directive(tokens, "e2m3x2");
                            }
                            F6x2type::E3m2x2 => {
                                    push_directive(tokens, "e3m2x2");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRsReluSatfiniteF6x4typeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rs");
                    if self.relu {
                            push_directive(tokens, "relu");
                    }
                    push_directive(tokens, "satfinite");
                    match &self.f6x4type {
                            F6x4type::E2m3x4 => {
                                    push_directive(tokens, "e2m3x4");
                            }
                            F6x4type::E3m2x4 => {
                                    push_directive(tokens, "e3m2x4");
                            }
                    }
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBrace);
                    let &( ref group_4_0, ref group_4_1, ref group_4_2, ref group_4_3) = &self.a;
                    group_4_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_4_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_4_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_4_3.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBrace);
            tokens.push(PtxToken::Comma);
                    self.rbits.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd3SatfiniteUe8m0x2F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd3 {
                            Frnd3::Rz => {
                                    push_directive(tokens, "rz");
                            }
                            Frnd3::Rp => {
                                    push_directive(tokens, "rp");
                            }
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "ue8m0x2");
                    push_directive(tokens, "f32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtFrnd3SatfiniteUe8m0x2Bf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    match &self.frnd3 {
                            Frnd3::Rz => {
                                    push_directive(tokens, "rz");
                            }
                            Frnd3::Rp => {
                                    push_directive(tokens, "rp");
                            }
                    }
                    if self.satfinite {
                            push_directive(tokens, "satfinite");
                    }
                    push_directive(tokens, "ue8m0x2");
                    push_directive(tokens, "bf16x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CvtRnBf16x2Ue8m0x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cvt");
                    push_directive(tokens, "rn");
                    push_directive(tokens, "bf16x2");
                    push_directive(tokens, "ue8m0x2");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

