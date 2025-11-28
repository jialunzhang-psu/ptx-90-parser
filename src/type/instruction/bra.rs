//! Original PTX specification:
//!
//! bra{.uni}  tgt;           // tgt is a label
//! bra{.uni}  tgt;           // unconditional branch

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct BraUni {
        pub uni: bool,           // {.uni}
        pub tgt: GeneralOperand, // tgt
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct BraUni1 {
        pub uni: bool,           // {.uni}
        pub tgt: GeneralOperand, // tgt
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BraUni;
pub use section_0::BraUni1;
