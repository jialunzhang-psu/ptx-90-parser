//! Original PTX specification:
//!
//! madc.hilo{.cc}.type  d, a, b, c;
//! .type = { .u32, .s32, .u64, .s64 };
//! .hilo = { .hi, .lo };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Hilo {
        Hi, // .hi
        Lo, // .lo
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MadcHiloCcType {
        pub hilo: Hilo,        // .hilo
        pub cc: bool,          // {.cc}
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Hilo as Hilo0;
pub use section_0::MadcHiloCcType;
pub use section_0::Type as Type0;
