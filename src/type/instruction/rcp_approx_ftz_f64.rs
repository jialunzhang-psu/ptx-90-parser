//! Original PTX specification:
//!
//! rcp.approx.ftz.f64  d, a;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct RcpApproxFtzF64 {
        pub approx: (),        // .approx
        pub ftz: (),           // .ftz
        pub f64: (),           // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::RcpApproxFtzF64;
