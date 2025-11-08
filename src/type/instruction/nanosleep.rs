//! Original PTX specification:
//!
//! nanosleep.u32 t;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct NanosleepU32 {
        pub u32: (),           // .u32
        pub t: GeneralOperand, // t
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::NanosleepU32;
