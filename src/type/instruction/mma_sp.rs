//! Original PTX specification:
//!
//! // Half precision floating point type:
//! mma.spvariant.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype  d, a, b, c, e, f;
//! mma.spvariant.sync.aligned.m16n8k32.row.col.dtype.f16.f16.ctype  d, a, b, c, e, f;
//! .ctype     = {.f16, .f32};
//! .dtype     = {.f16, .f32};
//! .spvariant = {.sp, .sp::ordered_metadata};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.spvariant.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32     d, a, b, c, e, f;
//! mma.spvariant.sync.aligned.m16n8k32.row.col.f32.bf16.bf16.f32     d, a, b, c, e, f;
//! mma.spvariant.sync.aligned.m16n8k8.row.col.f32.tf32.tf32.f32      d, a, b, c, e, f;
//! mma.spvariant.sync.aligned.m16n8k16.row.col.f32.tf32.tf32.f32     d, a, b, c, e, f;
//! mma.spvariant.sync.aligned.m16n8k64.row.col.f32.f8type.f8type.f32 d, a, b, c, e, f;
//! mma.sp::ordered_metadata.sync.aligned.m16n8k64.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c, e, f;
//! .f8type     = {.e4m3, .e5m2};
//! .spvariant  = {.sp, .sp::ordered_metadata};
//! .f8f6f4type = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .kind       = {.kind::f8f6f4};
//! .ctype      = {.f16, .f32};
//! .dtype      = {.f16, .f32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type with block scaling:
//! mma.spvariant.sync.aligned.m16n8k128.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, e, f, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .spvariant      = {.sp::ordered_metadata};
//! .kind           = {.kind::mxf4};
//! .scale_vec_size = {.scale_vec::2X};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.spvariant.sync.aligned.m16n8k128.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, e, f, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .spvariant      = {.sp::ordered_metadata};
//! .kind           = {.kind::mxf4nvf4};
//! .scale_vec_size = {.scale_vec::2X, .scale_vec::4X};
//! .stype          = {.ue8m0, .ue4m3};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.spvariant.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, e, f, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .spvariant      = {.sp::ordered_metadata};
//! .kind           = {.kind::mxf8f6f4};
//! .scale_vec_size = {.scale_vec::1X};
//! .f8f6f4type     = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Integer type:
//! mma.spvariant.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c, e, f;
//! .shape     = {.m16n8k32, .m16n8k64};
//! .atype     = {.u8, .s8};
//! .btype     = {.u8, .s8};
//! .spvariant = {.sp, .sp::ordered_metadata};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.spvariant.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c, e, f;
//! .shape     = {.m16n8k64, .m16n8k128};
//! .atype     = {.u4, .s4};
//! .btype     = {.u4, .s4};
//! .spvariant = {.sp, .sp::ordered_metadata};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
        Sp,                // .sp
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
    pub struct MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k16: (),         // .m16n8k16
        pub row: (),              // .row
        pub col: (),              // .col
        pub dtype: Dtype,         // .dtype
        pub f16: (),              // .f16
        pub f162: (),             // .f16
        pub ctype: Ctype,         // .ctype
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k32: (),         // .m16n8k32
        pub row: (),              // .row
        pub col: (),              // .col
        pub dtype: Dtype,         // .dtype
        pub f16: (),              // .f16
        pub f162: (),             // .f16
        pub ctype: Ctype,         // .ctype
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
        Sp,                // .sp
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum F8type {
        E4m3, // .e4m3
        E5m2, // .e5m2
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        F16, // .f16
        F32, // .f32
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
    pub enum Ctype {
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k16: (),         // .m16n8k16
        pub row: (),              // .row
        pub col: (),              // .col
        pub f32: (),              // .f32
        pub bf16: (),             // .bf16
        pub bf162: (),            // .bf16
        pub f322: (),             // .f32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k32: (),         // .m16n8k32
        pub row: (),              // .row
        pub col: (),              // .col
        pub f32: (),              // .f32
        pub bf16: (),             // .bf16
        pub bf162: (),            // .bf16
        pub f322: (),             // .f32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k8: (),          // .m16n8k8
        pub row: (),              // .row
        pub col: (),              // .col
        pub f32: (),              // .f32
        pub tf32: (),             // .tf32
        pub tf322: (),            // .tf32
        pub f322: (),             // .f32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k16: (),         // .m16n8k16
        pub row: (),              // .row
        pub col: (),              // .col
        pub f32: (),              // .f32
        pub tf32: (),             // .tf32
        pub tf322: (),            // .tf32
        pub f322: (),             // .f32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub m16n8k64: (),         // .m16n8k64
        pub row: (),              // .row
        pub col: (),              // .col
        pub f32: (),              // .f32
        pub f8type: F8type,       // .f8type
        pub f8type1: F8type,      // .f8type
        pub f322: (),             // .f32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
        pub sp_ordered_metadata: (), // .sp::ordered_metadata
        pub sync: (),                // .sync
        pub aligned: (),             // .aligned
        pub m16n8k64: (),            // .m16n8k64
        pub row: (),                 // .row
        pub col: (),                 // .col
        pub kind: Kind,              // .kind
        pub dtype: Dtype,            // .dtype
        pub f8f6f4type: F8f6f4type,  // .f8f6f4type
        pub f8f6f4type1: F8f6f4type, // .f8f6f4type
        pub ctype: Ctype,            // .ctype
        pub d: GeneralOperand,       // d
        pub a: GeneralOperand,       // a
        pub b: GeneralOperand,       // b
        pub c: GeneralOperand,       // c
        pub e: GeneralOperand,       // e
        pub f: GeneralOperand,       // f
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
    }

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
    pub struct MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
        pub spvariant: Spvariant,                 // .spvariant
        pub sync: (),                             // .sync
        pub aligned: (),                          // .aligned
        pub m16n8k128: (),                        // .m16n8k128
        pub row: (),                              // .row
        pub col: (),                              // .col
        pub kind: Kind,                           // .kind
        pub block_scale: (),                      // .block_scale
        pub scale_vec_size: Option<ScaleVecSize>, // {.scale_vec_size}
        pub f32: (),                              // .f32
        pub e2m1: (),                             // .e2m1
        pub e2m12: (),                            // .e2m1
        pub f322: (),                             // .f32
        pub stype: Stype,                         // .stype
        pub d: GeneralOperand,                    // d
        pub a: GeneralOperand,                    // a
        pub b: GeneralOperand,                    // b
        pub c: GeneralOperand,                    // c
        pub e: GeneralOperand,                    // e
        pub f: GeneralOperand,                    // f
        pub scale_a_data: GeneralOperand,         // scale-a-data
        pub byte_id_a: VectorOperand,             // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand,         // scale-b-data
        pub byte_id_b: VectorOperand,             // {byte-id-b, thread-id-b}
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
    }

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
    pub struct MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
        pub spvariant: Spvariant,         // .spvariant
        pub sync: (),                     // .sync
        pub aligned: (),                  // .aligned
        pub m16n8k128: (),                // .m16n8k128
        pub row: (),                      // .row
        pub col: (),                      // .col
        pub kind: Kind,                   // .kind
        pub block_scale: (),              // .block_scale
        pub scale_vec_size: ScaleVecSize, // .scale_vec_size
        pub f32: (),                      // .f32
        pub e2m1: (),                     // .e2m1
        pub e2m12: (),                    // .e2m1
        pub f322: (),                     // .f32
        pub stype: Stype,                 // .stype
        pub d: GeneralOperand,            // d
        pub a: GeneralOperand,            // a
        pub b: GeneralOperand,            // b
        pub c: GeneralOperand,            // c
        pub e: GeneralOperand,            // e
        pub f: GeneralOperand,            // f
        pub scale_a_data: GeneralOperand, // scale-a-data
        pub byte_id_a: VectorOperand,     // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand, // scale-b-data
        pub byte_id_b: VectorOperand,     // {byte-id-b, thread-id-b}
        pub span: Span,
    }
}

pub mod section_4 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
    }

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
    pub struct MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype
    {
        pub spvariant: Spvariant,                 // .spvariant
        pub sync: (),                             // .sync
        pub aligned: (),                          // .aligned
        pub m16n8k64: (),                         // .m16n8k64
        pub row: (),                              // .row
        pub col: (),                              // .col
        pub kind: Kind,                           // .kind
        pub block_scale: (),                      // .block_scale
        pub scale_vec_size: Option<ScaleVecSize>, // {.scale_vec_size}
        pub f32: (),                              // .f32
        pub f8f6f4type: F8f6f4type,               // .f8f6f4type
        pub f8f6f4type1: F8f6f4type,              // .f8f6f4type
        pub f322: (),                             // .f32
        pub stype: Stype,                         // .stype
        pub d: GeneralOperand,                    // d
        pub a: GeneralOperand,                    // a
        pub b: GeneralOperand,                    // b
        pub c: GeneralOperand,                    // c
        pub e: GeneralOperand,                    // e
        pub f: GeneralOperand,                    // f
        pub scale_a_data: GeneralOperand,         // scale-a-data
        pub byte_id_a: VectorOperand,             // {byte-id-a, thread-id-a}
        pub scale_b_data: GeneralOperand,         // scale-b-data
        pub byte_id_b: VectorOperand,             // {byte-id-b, thread-id-b}
        pub span: Span,
    }
}

pub mod section_5 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
        Sp,                // .sp
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k32, // .m16n8k32
        M16n8k64, // .m16n8k64
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
    pub struct MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub shape: Shape,         // .shape
        pub row: (),              // .row
        pub col: (),              // .col
        pub satfinite: bool,      // {.satfinite}
        pub s32: (),              // .s32
        pub atype: Atype,         // .atype
        pub btype: Btype,         // .btype
        pub s322: (),             // .s32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }
}

pub mod section_6 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Spvariant {
        SpOrderedMetadata, // .sp::ordered_metadata
        Sp,                // .sp
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M16n8k128, // .m16n8k128
        M16n8k64,  // .m16n8k64
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
    pub struct MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
        pub spvariant: Spvariant, // .spvariant
        pub sync: (),             // .sync
        pub aligned: (),          // .aligned
        pub shape: Shape,         // .shape
        pub row: (),              // .row
        pub col: (),              // .col
        pub satfinite: bool,      // {.satfinite}
        pub s32: (),              // .s32
        pub atype: Atype,         // .atype
        pub btype: Btype,         // .btype
        pub s322: (),             // .s32
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub b: GeneralOperand,    // b
        pub c: GeneralOperand,    // c
        pub e: GeneralOperand,    // e
        pub f: GeneralOperand,    // f
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Ctype as Ctype0;
pub use section_0::Dtype as Dtype0;
pub use section_0::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype;
pub use section_0::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype;
pub use section_0::Spvariant as Spvariant0;
pub use section_1::Ctype as Ctype1;
pub use section_1::Dtype as Dtype1;
pub use section_1::F8f6f4type as F8f6f4type1;
pub use section_1::F8type as F8type1;
pub use section_1::Kind as Kind1;
pub use section_1::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype;
pub use section_1::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32;
pub use section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32;
pub use section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32;
pub use section_1::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32;
pub use section_1::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32;
pub use section_1::Spvariant as Spvariant1;
pub use section_2::Kind as Kind2;
pub use section_2::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype;
pub use section_2::ScaleVecSize as ScaleVecSize2;
pub use section_2::Spvariant as Spvariant2;
pub use section_2::Stype as Stype2;
pub use section_3::Kind as Kind3;
pub use section_3::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1;
pub use section_3::ScaleVecSize as ScaleVecSize3;
pub use section_3::Spvariant as Spvariant3;
pub use section_3::Stype as Stype3;
pub use section_4::F8f6f4type as F8f6f4type4;
pub use section_4::Kind as Kind4;
pub use section_4::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype;
pub use section_4::ScaleVecSize as ScaleVecSize4;
pub use section_4::Spvariant as Spvariant4;
pub use section_4::Stype as Stype4;
pub use section_5::Atype as Atype5;
pub use section_5::Btype as Btype5;
pub use section_5::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32;
pub use section_5::Shape as Shape5;
pub use section_5::Spvariant as Spvariant5;
pub use section_6::Atype as Atype6;
pub use section_6::Btype as Btype6;
pub use section_6::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321;
pub use section_6::Shape as Shape6;
pub use section_6::Spvariant as Spvariant6;
