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
        pub sync: (), // .sync
        pub mode: Mode, // .mode
        pub pred: (), // .pred
        pub d: Operand, // d
        pub a_op: bool, // {!} operator
        pub a: Operand, // {!}a
        pub membermask: Operand, // membermask
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VoteSyncBallotB32 {
        pub sync: (), // .sync
        pub ballot: (), // .ballot
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a_op: bool, // {!} operator
        pub a: Operand, // {!}a
        pub membermask: Operand, // membermask
    }

}
