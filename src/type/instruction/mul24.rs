//! Original PTX specification:
//!
//! mul24.mode.type  d, a, b;
//! .mode = { .hi, .lo };
//! .type = { .u32, .s32 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

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
    pub struct Mul24ModeType {
        pub mode: Mode, // .mode
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mul24ModeType;
pub use section_0::Mode as Mode0;
pub use section_0::Type as Type0;
