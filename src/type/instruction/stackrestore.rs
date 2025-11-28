//! Original PTX specification:
//!
//! stackrestore.type  a;
//! .type = { .u32, .u64 };

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
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct StackrestoreType {
        pub type_: Type, // .type
        pub a: GeneralOperand, // a
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::StackrestoreType;
pub use section_0::Type as Type0;
