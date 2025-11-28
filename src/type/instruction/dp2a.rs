//! Original PTX specification:
//!
//! dp2a.mode.atype.btype  d, a, b, c;
//! .atype = .btype = { .u32, .s32 };
//! .mode = { .lo, .hi };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        Lo, // .lo
        Hi, // .hi
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Atype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Btype {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Dp2aModeAtypeBtype {
        pub mode: Mode,        // .mode
        pub atype: Atype,      // .atype
        pub btype: Btype,      // .btype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Atype as Atype0;
pub use section_0::Btype as Btype0;
pub use section_0::Dp2aModeAtypeBtype;
pub use section_0::Mode as Mode0;
