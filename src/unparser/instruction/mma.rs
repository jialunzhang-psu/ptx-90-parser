//! Original PTX specification:
//!
//! // Half precision floating point type:
//! mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .ctype   = {.f16, .f32};
//! .dtype   = {.f16, .f32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32        d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32      d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32       d, a, b, c;
//! mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;
//! .atype      = {.bf16, .tf32};
//! .btype      = {.bf16, .tf32};
//! .f8type     = {.e4m3, .e5m2};
//! .f8f6f4type = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .ctype      = {.f16, .f32};
//! .dtype      = {.f16, .f32};
//! .shape      = {.m16n8k16, .m16n8k32};
//! .kind       = {.kind::f8f6f4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type with block scaling:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4};
//! .scale_vec_size = {.scale_vec::2X};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4nvf4};
//! .scale_vec_size = {.scale_vec::2X, .scale_vec::4X};
//! .stype          = {.ue8m0, .ue4m3};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf8f6f4};
//! .scale_vec_size = {.scale_vec::1X};
//! .f8f6f4type     = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Double precision floating point type:
//! mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;
//! .shape   = {.m8n84, .m16n8k4, .m16n8k8, .m16n8k16};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Integer type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k16, .m16n8k16, .m16n8k32};
//! .atype   = {.u8, .s8};
//! .btype   = {.u8, .s8};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k32, .m16n8k32, .m16n8k64};
//! .atype   = {.u4, .s4};
//! .btype   = {.u4, .s4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Single bit:
//! mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;
//! .bitOp = {.xor, .and};
//! .shape = {.m8n8k128, .m16n8k128, .m16n8k256};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mma::section_0::*;

    impl PtxUnparser for MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m8n8k4");
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
            match &self.dtype {
                Dtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            push_directive(tokens, "f16");
            push_directive(tokens, "f16");
            match &self.ctype {
                Ctype::F16 => {
                    push_directive(tokens, "f16");
                }
                Ctype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k8");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.dtype {
                Dtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            push_directive(tokens, "f16");
            push_directive(tokens, "f16");
            match &self.ctype {
                Ctype::F16 => {
                    push_directive(tokens, "f16");
                }
                Ctype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k16");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.dtype {
                Dtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            push_directive(tokens, "f16");
            push_directive(tokens, "f16");
            match &self.ctype {
                Ctype::F16 => {
                    push_directive(tokens, "f16");
                }
                Ctype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mma::section_1::*;

    impl PtxUnparser for MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k4");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            push_directive(tokens, "f32");
            push_directive(tokens, "tf32");
            push_directive(tokens, "tf32");
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k8");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            push_directive(tokens, "f32");
            match &self.atype {
                Atype::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Atype::Tf32 => {
                    push_directive(tokens, "tf32");
                }
            }
            match &self.btype {
                Btype::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Btype::Tf32 => {
                    push_directive(tokens, "tf32");
                }
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k16");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            push_directive(tokens, "f32");
            push_directive(tokens, "bf16");
            push_directive(tokens, "bf16");
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M16n8k16 => {
                    push_directive(tokens, "m16n8k16");
                }
                Shape::M16n8k32 => {
                    push_directive(tokens, "m16n8k32");
                }
            }
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.dtype {
                Dtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            match &self.f8type {
                F8type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
            }
            match &self.f8type1 {
                F8type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
            }
            match &self.ctype {
                Ctype::F16 => {
                    push_directive(tokens, "f16");
                }
                Ctype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k32");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.kind {
                Kind::KindF8f6f4 => {
                    push_directive(tokens, "kind::f8f6f4");
                }
            }
            match &self.dtype {
                Dtype::F16 => {
                    push_directive(tokens, "f16");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            match &self.f8f6f4type {
                F8f6f4type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8f6f4type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                F8f6f4type::E3m2 => {
                    push_directive(tokens, "e3m2");
                }
                F8f6f4type::E2m3 => {
                    push_directive(tokens, "e2m3");
                }
                F8f6f4type::E2m1 => {
                    push_directive(tokens, "e2m1");
                }
            }
            match &self.f8f6f4type1 {
                F8f6f4type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8f6f4type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                F8f6f4type::E3m2 => {
                    push_directive(tokens, "e3m2");
                }
                F8f6f4type::E2m3 => {
                    push_directive(tokens, "e2m3");
                }
                F8f6f4type::E2m1 => {
                    push_directive(tokens, "e2m1");
                }
            }
            match &self.ctype {
                Ctype::F16 => {
                    push_directive(tokens, "f16");
                }
                Ctype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::mma::section_2::*;

    impl PtxUnparser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k64");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.kind {
                Kind::KindMxf4 => {
                    push_directive(tokens, "kind::mxf4");
                }
            }
            push_directive(tokens, "block_scale");
            if let Some(scale_vec_size_0) = self.scale_vec_size.as_ref() {
                match scale_vec_size_0 {
                    ScaleVecSize::ScaleVec2x => {
                        push_directive(tokens, "scale_vec::2X");
                    }
                }
            }
            push_directive(tokens, "f32");
            push_directive(tokens, "e2m1");
            push_directive(tokens, "e2m1");
            push_directive(tokens, "f32");
            match &self.stype {
                Stype::Ue8m0 => {
                    push_directive(tokens, "ue8m0");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_a_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_b_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::mma::section_3::*;

    impl PtxUnparser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k64");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.kind {
                Kind::KindMxf4nvf4 => {
                    push_directive(tokens, "kind::mxf4nvf4");
                }
            }
            push_directive(tokens, "block_scale");
            match &self.scale_vec_size {
                ScaleVecSize::ScaleVec2x => {
                    push_directive(tokens, "scale_vec::2X");
                }
                ScaleVecSize::ScaleVec4x => {
                    push_directive(tokens, "scale_vec::4X");
                }
            }
            push_directive(tokens, "f32");
            push_directive(tokens, "e2m1");
            push_directive(tokens, "e2m1");
            push_directive(tokens, "f32");
            match &self.stype {
                Stype::Ue8m0 => {
                    push_directive(tokens, "ue8m0");
                }
                Stype::Ue4m3 => {
                    push_directive(tokens, "ue4m3");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_a_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_b_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::mma::section_4::*;

    impl PtxUnparser
        for MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype
    {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            push_directive(tokens, "m16n8k32");
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            match &self.kind {
                Kind::KindMxf8f6f4 => {
                    push_directive(tokens, "kind::mxf8f6f4");
                }
            }
            push_directive(tokens, "block_scale");
            if let Some(scale_vec_size_1) = self.scale_vec_size.as_ref() {
                match scale_vec_size_1 {
                    ScaleVecSize::ScaleVec1x => {
                        push_directive(tokens, "scale_vec::1X");
                    }
                }
            }
            push_directive(tokens, "f32");
            match &self.f8f6f4type {
                F8f6f4type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8f6f4type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                F8f6f4type::E3m2 => {
                    push_directive(tokens, "e3m2");
                }
                F8f6f4type::E2m3 => {
                    push_directive(tokens, "e2m3");
                }
                F8f6f4type::E2m1 => {
                    push_directive(tokens, "e2m1");
                }
            }
            match &self.f8f6f4type1 {
                F8f6f4type::E4m3 => {
                    push_directive(tokens, "e4m3");
                }
                F8f6f4type::E5m2 => {
                    push_directive(tokens, "e5m2");
                }
                F8f6f4type::E3m2 => {
                    push_directive(tokens, "e3m2");
                }
                F8f6f4type::E2m3 => {
                    push_directive(tokens, "e2m3");
                }
                F8f6f4type::E2m1 => {
                    push_directive(tokens, "e2m1");
                }
            }
            push_directive(tokens, "f32");
            match &self.stype {
                Stype::Ue8m0 => {
                    push_directive(tokens, "ue8m0");
                }
            }
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_a_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.scale_b_data.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.byte_id_b.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::mma::section_5::*;

    impl PtxUnparser for MmaSyncAlignedShapeRowColF64F64F64F64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M16n8k16 => {
                    push_directive(tokens, "m16n8k16");
                }
                Shape::M16n8k4 => {
                    push_directive(tokens, "m16n8k4");
                }
                Shape::M16n8k8 => {
                    push_directive(tokens, "m16n8k8");
                }
                Shape::M8n84 => {
                    push_directive(tokens, "m8n84");
                }
            }
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            push_directive(tokens, "f64");
            push_directive(tokens, "f64");
            push_directive(tokens, "f64");
            push_directive(tokens, "f64");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_6 {
    use super::*;
    use crate::r#type::instruction::mma::section_6::*;

    impl PtxUnparser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M16n8k16 => {
                    push_directive(tokens, "m16n8k16");
                }
                Shape::M16n8k32 => {
                    push_directive(tokens, "m16n8k32");
                }
                Shape::M8n8k16 => {
                    push_directive(tokens, "m8n8k16");
                }
            }
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            if self.satfinite {
                push_directive(tokens, "satfinite");
            }
            push_directive(tokens, "s32");
            match &self.atype {
                Atype::U8 => {
                    push_directive(tokens, "u8");
                }
                Atype::S8 => {
                    push_directive(tokens, "s8");
                }
            }
            match &self.btype {
                Btype::U8 => {
                    push_directive(tokens, "u8");
                }
                Btype::S8 => {
                    push_directive(tokens, "s8");
                }
            }
            push_directive(tokens, "s32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_7 {
    use super::*;
    use crate::r#type::instruction::mma::section_7::*;

    impl PtxUnparser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M16n8k32 => {
                    push_directive(tokens, "m16n8k32");
                }
                Shape::M16n8k64 => {
                    push_directive(tokens, "m16n8k64");
                }
                Shape::M8n8k32 => {
                    push_directive(tokens, "m8n8k32");
                }
            }
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            if self.satfinite {
                push_directive(tokens, "satfinite");
            }
            push_directive(tokens, "s32");
            match &self.atype {
                Atype::U4 => {
                    push_directive(tokens, "u4");
                }
                Atype::S4 => {
                    push_directive(tokens, "s4");
                }
            }
            match &self.btype {
                Btype::U4 => {
                    push_directive(tokens, "u4");
                }
                Btype::S4 => {
                    push_directive(tokens, "s4");
                }
            }
            push_directive(tokens, "s32");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

pub mod section_8 {
    use super::*;
    use crate::r#type::instruction::mma::section_8::*;

    impl PtxUnparser for MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "mma");
            push_directive(tokens, "sync");
            push_directive(tokens, "aligned");
            match &self.shape {
                Shape::M16n8k128 => {
                    push_directive(tokens, "m16n8k128");
                }
                Shape::M16n8k256 => {
                    push_directive(tokens, "m16n8k256");
                }
                Shape::M8n8k128 => {
                    push_directive(tokens, "m8n8k128");
                }
            }
            push_directive(tokens, "row");
            push_directive(tokens, "col");
            push_directive(tokens, "s32");
            push_directive(tokens, "b1");
            push_directive(tokens, "b1");
            push_directive(tokens, "s32");
            match &self.bitop {
                Bitop::Xor => {
                    push_directive(tokens, "xor");
                }
                Bitop::And => {
                    push_directive(tokens, "and");
                }
            }
            push_directive(tokens, "popc");
            self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.b.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
