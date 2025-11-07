//! Original PTX specification:
//!
//! match.any.sync.type  d, a, membermask;
//! match.all.sync.type  d{|p}, a, membermask;
//! .type = { .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MatchAnySyncType {
        pub any: (), // .any
        pub sync: (), // .sync
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub membermask: GeneralOperand, // membermask
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MatchAllSyncType {
        pub all: (), // .all
        pub sync: (), // .sync
        pub type_: Type, // .type
        pub d: GeneralOperand, // first operand of d{|p}
        pub p: Option<GeneralOperand>, // optional second operand of d{|p}
        pub a: GeneralOperand, // a
        pub membermask: GeneralOperand, // membermask
    }

}
