//! Original PTX specification:
//!
//! bfind.type           d, a;
//! bfind.shiftamt.type  d, a;
//! .type = { .u32, .u64, .s32, .s64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct BfindType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct BfindShiftamtType {
        pub shiftamt: (),      // .shiftamt
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BfindShiftamtType;
pub use section_0::BfindType;
pub use section_0::Type as Type0;
