//! Original PTX specification:
//!
//! xor.type d, a, b;
//! .type = { .pred, .b16, .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Pred, // .pred
        B16,  // .b16
        B32,  // .b32
        B64,  // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct XorType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Type as Type0;
pub use section_0::XorType;
