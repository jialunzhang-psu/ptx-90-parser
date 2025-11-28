//! Original PTX specification:
//!
//! slct.dtype.s32        d, a, b, c;
//! slct{.ftz}.dtype.f32  d, a, b, c;
//! .dtype = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Dtype {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SlctDtypeS32 {
        pub dtype: Dtype, // .dtype
        pub s32: (), // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SlctFtzDtypeF32 {
        pub ftz: bool, // {.ftz}
        pub dtype: Dtype, // .dtype
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SlctDtypeS32;
pub use section_0::SlctFtzDtypeF32;
pub use section_0::Dtype as Dtype0;
