//! Original PTX specification:
//!
//! shfl.mode.b32  d{|p}, a, b, c;
//! .mode = { .up, .down, .bfly, .idx };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Down, // .down
        Bfly, // .bfly
        Idx,  // .idx
        Up,   // .up
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct ShflModeB32 {
        pub mode: Mode,                // .mode
        pub b32: (),                   // .b32
        pub d: GeneralOperand,         // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: GeneralOperand,         // a
        pub b: GeneralOperand,         // b
        pub c: GeneralOperand,         // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::ShflModeB32;
