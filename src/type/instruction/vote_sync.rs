//! Original PTX specification:
//!
//! vote.sync.mode.pred  d, {!}a, membermask;
//! vote.sync.ballot.b32 d, {!}a, membermask;  // 'ballot' form, returns bitmask
//! .mode = { .all, .any, .uni };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        All, // .all
        Any, // .any
        Uni, // .uni
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VoteSyncModePred {
        pub sync: (),                   // .sync
        pub mode: Mode,                 // .mode
        pub pred: (),                   // .pred
        pub d: GeneralOperand,          // d
        pub a_op: bool,                 // {!} operator
        pub a: GeneralOperand,          // {!}a
        pub membermask: GeneralOperand, // membermask
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VoteSyncBallotB32 {
        pub sync: (),                   // .sync
        pub ballot: (),                 // .ballot
        pub b32: (),                    // .b32
        pub d: GeneralOperand,          // d
        pub a_op: bool,                 // {!} operator
        pub a: GeneralOperand,          // {!}a
        pub membermask: GeneralOperand, // membermask
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Mode as Mode0;
pub use section_0::VoteSyncBallotB32;
pub use section_0::VoteSyncModePred;
