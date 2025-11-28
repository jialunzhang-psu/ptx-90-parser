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
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Alayout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Blayout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m8n8k4: (), // .m8n8k4
        pub alayout: Alayout, // .alayout
        pub blayout: Blayout, // .blayout
        pub dtype: Dtype, // .dtype
        pub f16: (), // .f16
        pub f162: (), // .f16
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k8: (), // .m16n8k8
        pub row: (), // .row
        pub col: (), // .col
        pub dtype: Dtype, // .dtype
        pub f16: (), // .f16
        pub f162: (), // .f16
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k16: (), // .m16n8k16
        pub row: (), // .row
        pub col: (), // .col
        pub dtype: Dtype, // .dtype
        pub f16: (), // .f16
        pub f162: (), // .f16
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_1 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        Bf16, // .bf16
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        Bf16, // .bf16
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k16, // .m16n8k16
        M16n8k32, // .m16n8k32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum F8type {
        E4m3, // .e4m3
        E5m2, // .e5m2
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ctype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum F8f6f4type {
        E4m3, // .e4m3
        E5m2, // .e5m2
        E3m2, // .e3m2
        E2m3, // .e2m3
        E2m1, // .e2m1
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k4: (), // .m16n8k4
        pub row: (), // .row
        pub col: (), // .col
        pub f32: (), // .f32
        pub tf32: (), // .tf32
        pub tf322: (), // .tf32
        pub f322: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k8: (), // .m16n8k8
        pub row: (), // .row
        pub col: (), // .col
        pub f32: (), // .f32
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub f322: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k16: (), // .m16n8k16
        pub row: (), // .row
        pub col: (), // .col
        pub f32: (), // .f32
        pub bf16: (), // .bf16
        pub bf162: (), // .bf16
        pub f322: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub row: (), // .row
        pub col: (), // .col
        pub dtype: Dtype, // .dtype
        pub f8type: F8type, // .f8type
        pub f8type1: F8type, // .f8type
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k32: (), // .m16n8k32
        pub row: (), // .row
        pub col: (), // .col
        pub kind: Kind, // .kind
        pub dtype: Dtype, // .dtype
        pub f8f6f4type: F8f6f4type, // .f8f6f4type
        pub f8f6f4type1: F8f6f4type, // .f8f6f4type
        pub ctype: Ctype, // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_2 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindMxf4, // .kind::mxf4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum ScaleVecSize {
        ScaleVec2x, // .scale_vec::2X
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Stype {
        Ue8m0, // .ue8m0
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k64: (), // .m16n8k64
        pub row: (), // .row
        pub col: (), // .col
        pub kind: Kind, // .kind
        pub block_scale: (), // .block_scale
        pub scale_vec_size: Option<ScaleVecSize>, // {.scale_vec_size}
        pub f32: (), // .f32
        pub e2m1: (), // .e2m1
        pub e2m12: (), // .e2m1
        pub f322: (), // .f32
        pub stype: Stype, // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub scale_a_data: GeneralOperand, // scale-a-data
        pub byte_id_a: VectorOperand, // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand, // scale-b-data
        pub byte_id_b: VectorOperand, // {byte-id-b, thread-id-b}
        pub span: Span,
    }

}

pub mod section_3 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindMxf4nvf4, // .kind::mxf4nvf4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum ScaleVecSize {
        ScaleVec2x, // .scale_vec::2X
        ScaleVec4x, // .scale_vec::4X
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Stype {
        Ue8m0, // .ue8m0
        Ue4m3, // .ue4m3
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k64: (), // .m16n8k64
        pub row: (), // .row
        pub col: (), // .col
        pub kind: Kind, // .kind
        pub block_scale: (), // .block_scale
        pub scale_vec_size: ScaleVecSize, // .scale_vec_size
        pub f32: (), // .f32
        pub e2m1: (), // .e2m1
        pub e2m12: (), // .e2m1
        pub f322: (), // .f32
        pub stype: Stype, // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub scale_a_data: GeneralOperand, // scale-a-data
        pub byte_id_a: VectorOperand, // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand, // scale-b-data
        pub byte_id_b: VectorOperand, // {byte-id-b, thread-id-b}
        pub span: Span,
    }

}

pub mod section_4 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindMxf8f6f4, // .kind::mxf8f6f4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum ScaleVecSize {
        ScaleVec1x, // .scale_vec::1X
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum F8f6f4type {
        E4m3, // .e4m3
        E5m2, // .e5m2
        E3m2, // .e3m2
        E2m3, // .e2m3
        E2m1, // .e2m1
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Stype {
        Ue8m0, // .ue8m0
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub m16n8k32: (), // .m16n8k32
        pub row: (), // .row
        pub col: (), // .col
        pub kind: Kind, // .kind
        pub block_scale: (), // .block_scale
        pub scale_vec_size: Option<ScaleVecSize>, // {.scale_vec_size}
        pub f32: (), // .f32
        pub f8f6f4type: F8f6f4type, // .f8f6f4type
        pub f8f6f4type1: F8f6f4type, // .f8f6f4type
        pub f322: (), // .f32
        pub stype: Stype, // .stype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub scale_a_data: GeneralOperand, // scale-a-data
        pub byte_id_a: VectorOperand, // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand, // scale-b-data
        pub byte_id_b: VectorOperand, // {byte-id-b, thread-id-b}
        pub span: Span,
    }

}

pub mod section_5 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k16, // .m16n8k16
        M16n8k4, // .m16n8k4
        M16n8k8, // .m16n8k8
        M8n84, // .m8n84
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedShapeRowColF64F64F64F64 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub row: (), // .row
        pub col: (), // .col
        pub f64: (), // .f64
        pub f642: (), // .f64
        pub f644: (), // .f64
        pub f646: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_6 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k16, // .m16n8k16
        M16n8k32, // .m16n8k32
        M8n8k16, // .m8n8k16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub row: (), // .row
        pub col: (), // .col
        pub satfinite: bool, // {.satfinite}
        pub s32: (), // .s32
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub s322: (), // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_7 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k32, // .m16n8k32
        M16n8k64, // .m16n8k64
        M8n8k32, // .m8n8k32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        U4, // .u4
        S4, // .s4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        U4, // .u4
        S4, // .s4
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub row: (), // .row
        pub col: (), // .col
        pub satfinite: bool, // {.satfinite}
        pub s32: (), // .s32
        pub atype: Atype, // .atype
        pub btype: Btype, // .btype
        pub s322: (), // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

pub mod section_8 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k128, // .m16n8k128
        M16n8k256, // .m16n8k256
        M8n8k128, // .m8n8k128
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Bitop {
        Xor, // .xor
        And, // .and
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub row: (), // .row
        pub col: (), // .col
        pub s32: (), // .s32
        pub b1: (), // .b1
        pub b12: (), // .b1
        pub s322: (), // .s32
        pub bitop: Bitop, // .bitOp
        pub popc: (), // .popc
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype;
pub use section_0::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype;
pub use section_0::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype;
pub use section_0::Alayout as Alayout0;
pub use section_0::Blayout as Blayout0;
pub use section_0::Dtype as Dtype0;
pub use section_0::Ctype as Ctype0;
pub use section_1::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32;
pub use section_1::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32;
pub use section_1::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32;
pub use section_1::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype;
pub use section_1::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype;
pub use section_1::Atype as Atype1;
pub use section_1::Btype as Btype1;
pub use section_1::Shape as Shape1;
pub use section_1::Dtype as Dtype1;
pub use section_1::F8type as F8type1;
pub use section_1::Ctype as Ctype1;
pub use section_1::Kind as Kind1;
pub use section_1::F8f6f4type as F8f6f4type1;
pub use section_2::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype;
pub use section_2::Kind as Kind2;
pub use section_2::ScaleVecSize as ScaleVecSize2;
pub use section_2::Stype as Stype2;
pub use section_3::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1;
pub use section_3::Kind as Kind3;
pub use section_3::ScaleVecSize as ScaleVecSize3;
pub use section_3::Stype as Stype3;
pub use section_4::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype;
pub use section_4::Kind as Kind4;
pub use section_4::ScaleVecSize as ScaleVecSize4;
pub use section_4::F8f6f4type as F8f6f4type4;
pub use section_4::Stype as Stype4;
pub use section_5::MmaSyncAlignedShapeRowColF64F64F64F64;
pub use section_5::Shape as Shape5;
pub use section_6::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32;
pub use section_6::Shape as Shape6;
pub use section_6::Atype as Atype6;
pub use section_6::Btype as Btype6;
pub use section_7::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321;
pub use section_7::Shape as Shape7;
pub use section_7::Atype as Atype7;
pub use section_7::Btype as Btype7;
pub use section_8::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc;
pub use section_8::Shape as Shape8;
pub use section_8::Bitop as Bitop8;
