//! Original PTX specification:
//!
//! bmsk.mode.b32  d, a, b;
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
    pub struct BmskModeB32 {
        pub mode: Mode,        // .mode
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BmskModeB32;
pub use section_0::Mode as Mode0;
