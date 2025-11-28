//! Original PTX specification:
//!
//! min.atype         d, a, b;
//! min{.relu}.btype  d, a, b;
//! .atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 };
//! .btype = { .s16x2, .s32 };
//! 
//! min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
//! min{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
//! min.f64                            d, a, b;
//! 
//! min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
//! min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16           d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        U16x2, // .u16x2
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        S16x2, // .s16x2
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinAtype {
        pub atype: Atype, // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinReluBtype {
        pub relu: bool, // {.relu}
        pub btype: Btype, // .btype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinFtzNanXorsignAbsF32 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinFtzNanAbsF32 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub abs: bool, // {.abs}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinF64 {
        pub f64: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinFtzNanXorsignAbsF16 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f16: (), // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinFtzNanXorsignAbsF16x2 {
        pub ftz: bool, // {.ftz}
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub f16x2: (), // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinNanXorsignAbsBf16 {
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub bf16: (), // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MinNanXorsignAbsBf16x2 {
        pub nan: bool, // {.NaN}
        pub xorsign_abs: bool, // {.xorsign.abs}
        pub bf16x2: (), // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MinAtype;
pub use section_0::MinReluBtype;
pub use section_0::MinFtzNanXorsignAbsF32;
pub use section_0::MinFtzNanAbsF32;
pub use section_0::MinF64;
pub use section_0::MinFtzNanXorsignAbsF16;
pub use section_0::MinFtzNanXorsignAbsF16x2;
pub use section_0::MinNanXorsignAbsBf16;
pub use section_0::MinNanXorsignAbsBf16x2;
pub use section_0::Atype as Atype0;
pub use section_0::Btype as Btype0;
