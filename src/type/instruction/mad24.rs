//! Original PTX specification:
//!
//! mad24.mode.type  d, a, b, c;
//! mad24.hi.sat.s32 d, a, b, c;
//! .mode = { .hi, .lo };
//! .type = { .u32, .s32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Hi, // .hi
        Lo, // .lo
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Mad24ModeType {
        pub mode: Mode,        // .mode
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Mad24HiSatS32 {
        pub hi: (),            // .hi
        pub sat: (),           // .sat
        pub s32: (),           // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mad24HiSatS32;
pub use section_0::Mad24ModeType;
pub use section_0::Mode as Mode0;
pub use section_0::Type as Type0;
