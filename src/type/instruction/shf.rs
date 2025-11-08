//! Original PTX specification:
//!
//! shf.l.mode.b32  d, a, b, c;  // left shift
//! shf.r.mode.b32  d, a, b, c;  // right shift
//! .mode = { .clamp, .wrap };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Clamp, // .clamp
        Wrap,  // .wrap
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShfLModeB32 {
        pub l: (),             // .l
        pub mode: Mode,        // .mode
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShfRModeB32 {
        pub r: (),             // .r
        pub mode: Mode,        // .mode
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::ShfLModeB32;
pub use section_0::ShfRModeB32;
