//! Original PTX specification:
//!
//! stacksave.type  d;
//! .type = { .u32, .u64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StacksaveType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::StacksaveType;
pub use section_0::Type as Type0;
