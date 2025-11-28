//! Original PTX specification:
//!
//! bfe.type  d, a, b, c;
//! .type = { .u32, .u64, .s32, .s64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct BfeType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BfeType;
pub use section_0::Type as Type0;
