//! Original PTX specification:
//!
//! sqrt.approx{.ftz}.f32  d, a; // fast, approximate square root
//! sqrt.rnd{.ftz}.f32     d, a; // IEEE 754 compliant rounding
//! sqrt.rnd.f64           d, a; // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SqrtApproxFtzF32 {
        pub approx: (), // .approx
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SqrtRndFtzF32 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SqrtRndF64 {
        pub rnd: Rnd, // .rnd
        pub f64: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SqrtApproxFtzF32;
pub use section_0::SqrtRndFtzF32;
pub use section_0::SqrtRndF64;
pub use section_0::Rnd as Rnd0;
