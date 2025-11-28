//! Original PTX specification:
//!
//! brx.idx{.uni} index, tlist;
//! brx.idx{.uni} index, tlist;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct BrxIdxUni {
        pub idx: (), // .idx
        pub uni: bool, // {.uni}
        pub index: GeneralOperand, // index
        pub tlist: GeneralOperand, // tlist
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct BrxIdxUni1 {
        pub idx: (), // .idx
        pub uni: bool, // {.uni}
        pub index: GeneralOperand, // index
        pub tlist: GeneralOperand, // tlist
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BrxIdxUni;
pub use section_0::BrxIdxUni1;
