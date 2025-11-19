//! Original PTX specification:
//!
//! match.any.sync.type  d, a, membermask;
//! match.all.sync.type  d{|p}, a, membermask;
//! .type = { .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct MatchAnySyncType {
        pub any: (),                    // .any
        pub sync: (),                   // .sync
        pub type_: Type,                // .type
        pub d: GeneralOperand,          // d
        pub a: GeneralOperand,          // a
        pub membermask: GeneralOperand, // membermask
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct MatchAllSyncType {
        pub all: (),                    // .all
        pub sync: (),                   // .sync
        pub type_: Type,                // .type
        pub d: GeneralOperand,          // first operand of d{|p}
        pub p: Option<GeneralOperand>,  // optional second operand of d{|p}
        pub a: GeneralOperand,          // a
        pub membermask: GeneralOperand, // membermask
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MatchAllSyncType;
pub use section_0::MatchAnySyncType;
pub use section_0::Type as Type0;
