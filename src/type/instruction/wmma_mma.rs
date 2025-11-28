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
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub alayout: (),       // .alayout
        pub blayout: (),       // .blayout
        pub shape: (),         // .shape
        pub dtype: (),         // .dtype
        pub ctype: (),         // .ctype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

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
    pub enum Shape {
        M16n16k16, // .m16n16k16
        M8n32k16,  // .m8n32k16
        M32n8k16,  // .m32n8k16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        S8, // .s8
        U8, // .u8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        S8, // .s8
        U8, // .u8
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub alayout: Alayout,  // .alayout
        pub blayout: Blayout,  // .blayout
        pub shape: Shape,      // .shape
        pub s32: (),           // .s32
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub s322: (),          // .s32
        pub satfinite: bool,   // {.satfinite}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

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
    pub enum Shape {
        M16n16k16, // .m16n16k16
        M8n32k16,  // .m8n32k16
        M32n8k16,  // .m32n8k16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub alayout: Alayout,  // .alayout
        pub blayout: Blayout,  // .blayout
        pub shape: Shape,      // .shape
        pub f32: (),           // .f32
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub f322: (),          // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

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
    pub enum Shape {
        M16n16k8, // .m16n16k8
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub alayout: Alayout,  // .alayout
        pub blayout: Blayout,  // .blayout
        pub shape: Shape,      // .shape
        pub f32: (),           // .f32
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub f322: (),          // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_4 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

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
    pub enum Shape {
        M8n8k4, // .m8n8k4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub alayout: Alayout,  // .alayout
        pub blayout: Blayout,  // .blayout
        pub shape: Shape,      // .shape
        pub rnd: Option<Rnd>,  // {.rnd}
        pub f64: (),           // .f64
        pub f642: (),          // .f64
        pub f644: (),          // .f64
        pub f646: (),          // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_5 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M8n8k32, // .m8n8k32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        S4, // .s4
        U4, // .u4
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        S4, // .s4
        U4, // .u4
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
        pub mma: (),           // .mma
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub row: (),           // .row
        pub col: (),           // .col
        pub shape: Shape,      // .shape
        pub s32: (),           // .s32
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub s322: (),          // .s32
        pub satfinite: bool,   // {.satfinite}
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

pub mod section_6 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Xor, // .xor
        And, // .and
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Shape {
        M8n8k128, // .m8n8k128
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        B1, // .b1
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        B1, // .b1
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
        pub mma: (),           // .mma
        pub op: Op,            // .op
        pub popc: (),          // .popc
        pub sync: (),          // .sync
        pub aligned: (),       // .aligned
        pub row: (),           // .row
        pub col: (),           // .col
        pub shape: Shape,      // .shape
        pub s32: (),           // .s32
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub s322: (),          // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype;
pub use section_1::Alayout as Alayout1;
pub use section_1::Atype as Atype1;
pub use section_1::Blayout as Blayout1;
pub use section_1::Btype as Btype1;
pub use section_1::Shape as Shape1;
pub use section_1::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite;
pub use section_2::Alayout as Alayout2;
pub use section_2::Atype as Atype2;
pub use section_2::Blayout as Blayout2;
pub use section_2::Btype as Btype2;
pub use section_2::Shape as Shape2;
pub use section_2::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32;
pub use section_3::Alayout as Alayout3;
pub use section_3::Atype as Atype3;
pub use section_3::Blayout as Blayout3;
pub use section_3::Btype as Btype3;
pub use section_3::Shape as Shape3;
pub use section_3::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321;
pub use section_4::Alayout as Alayout4;
pub use section_4::Blayout as Blayout4;
pub use section_4::Rnd as Rnd4;
pub use section_4::Shape as Shape4;
pub use section_4::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64;
pub use section_5::Atype as Atype5;
pub use section_5::Btype as Btype5;
pub use section_5::Shape as Shape5;
pub use section_5::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite;
pub use section_6::Atype as Atype6;
pub use section_6::Btype as Btype6;
pub use section_6::Op as Op6;
pub use section_6::Shape as Shape6;
pub use section_6::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32;
