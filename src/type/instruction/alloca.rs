//! Original PTX specification:
//!
//! alloca.type  ptr, size{, immAlign};
//! .type = { .u32, .u64 };

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
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct AllocaType {
        pub type_: Type,                      // .type
        pub ptr: GeneralOperand,              // ptr
        pub size: GeneralOperand,             // size
        pub immalign: Option<GeneralOperand>, // {, immAlign}
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::AllocaType;
pub use section_0::Type as Type0;
