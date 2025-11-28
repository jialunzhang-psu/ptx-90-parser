//! Original PTX specification:
//!
//! szext.mode.type  d, a, b;
//! .mode = { .clamp, .wrap };
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
        Clamp, // .clamp
        Wrap,  // .wrap
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct SzextModeType {
        pub mode: Mode,        // .mode
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::SzextModeType;
pub use section_0::Type as Type0;
