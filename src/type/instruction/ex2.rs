//! Original PTX specification:
//!
//! ex2.approx{.ftz}.f32  d, a;
//!
//! ex2.approx.atype     d, a;
//! ex2.approx.ftz.btype d, a;
//! .atype = { .f16,  .f16x2};
//! .btype = { .bf16, .bf16x2};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        F16x2, // .f16x2
        F16,   // .f16
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        Bf16x2, // .bf16x2
        Bf16,   // .bf16
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Ex2ApproxFtzF32 {
        pub approx: (),        // .approx
        pub ftz: bool,         // {.ftz}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Ex2ApproxAtype {
        pub approx: (),        // .approx
        pub atype: Atype,      // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Ex2ApproxFtzBtype {
        pub approx: (),        // .approx
        pub ftz: (),           // .ftz
        pub btype: Btype,      // .btype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Atype as Atype0;
pub use section_0::Btype as Btype0;
pub use section_0::Ex2ApproxAtype;
pub use section_0::Ex2ApproxFtzBtype;
pub use section_0::Ex2ApproxFtzF32;
