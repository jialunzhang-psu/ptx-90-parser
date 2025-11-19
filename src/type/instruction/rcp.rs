//! Original PTX specification:
//!
//! rcp.approx{.ftz}.f32  d, a;  // fast, approximate reciprocal
//! rcp.rnd{.ftz}.f32     d, a;  // IEEE 754 compliant rounding
//! rcp.rnd.f64           d, a;  // IEEE 754 compliant rounding
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct RcpApproxFtzF32 {
        pub approx: (),        // .approx
        pub ftz: bool,         // {.ftz}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct RcpRndFtzF32 {
        pub rnd: Rnd,          // .rnd
        pub ftz: bool,         // {.ftz}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct RcpRndF64 {
        pub rnd: Rnd,          // .rnd
        pub f64: (),           // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::RcpApproxFtzF32;
pub use section_0::RcpRndF64;
pub use section_0::RcpRndFtzF32;
pub use section_0::Rnd as Rnd0;
