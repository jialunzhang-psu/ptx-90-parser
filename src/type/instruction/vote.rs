//! Original PTX specification:
//!
//! vote.mode.pred  d, {!}a;
//! vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask
//! .mode = { .all, .any, .uni };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Mode {
        All, // .all
        Any, // .any
        Uni, // .uni
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VoteModePred {
        pub mode: Mode,        // .mode
        pub pred: (),          // .pred
        pub d: GeneralOperand, // d
        pub a_op: bool,        // {!} operator
        pub a: GeneralOperand, // {!}a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct VoteBallotB32 {
        pub ballot: (),        // .ballot
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a_op: bool,        // {!} operator
        pub a: GeneralOperand, // {!}a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::VoteBallotB32;
pub use section_0::VoteModePred;
