//! Original PTX specification:
//!
//! tanh.approx.type d, a;
//! .type = {.f16, .f32, .f16x2, .bf16, .bf16x2};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        Bf16x2, // .bf16x2
        F16x2, // .f16x2
        Bf16, // .bf16
        F16, // .f16
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct TanhApproxType {
        pub approx: (), // .approx
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::TanhApproxType;
pub use section_0::Type as Type0;
