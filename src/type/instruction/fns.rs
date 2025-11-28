//! Original PTX specification:
//!
//! fns.b32 d, mask, base, offset;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct FnsB32 {
        pub b32: (),                // .b32
        pub d: GeneralOperand,      // d
        pub mask: GeneralOperand,   // mask
        pub base: GeneralOperand,   // base
        pub offset: GeneralOperand, // offset
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::FnsB32;
