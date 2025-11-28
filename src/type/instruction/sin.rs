//! Original PTX specification:
//!
//! sin.approx{.ftz}.f32  d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SinApproxFtzF32 {
        pub approx: (),        // .approx
        pub ftz: bool,         // {.ftz}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SinApproxFtzF32;
