//! Original PTX specification:
//!
//! vote.mode.pred  d, {!}a;
//! vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask
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
    pub struct VoteModePred {
        pub mode: Mode, // .mode
        pub pred: (), // .pred
        pub d: GeneralOperand, // d
        pub a_op: bool, // {!} operator
        pub a: GeneralOperand, // {!}a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VoteBallotB32 {
        pub ballot: (), // .ballot
        pub b32: (), // .b32
        pub d: GeneralOperand, // d
        pub a_op: bool, // {!} operator
        pub a: GeneralOperand, // {!}a
    }

}
