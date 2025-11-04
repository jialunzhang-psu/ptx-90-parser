//! Original PTX specification:
//!
//! redux.sync.op.type dst, src, membermask;
//! .op   = {.add, .min, .max};
//! .type = {.u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op.b32 dst, src, membermask;
//! .op   = {.and, .or, .xor};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op{.abs}{.NaN}.f32 dst, src, membermask;
//! .op   = { .min, .max };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ReduxSyncOpType {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub type_: Type, // .type
        pub dst: Operand, // dst
        pub src: Operand, // src
        pub membermask: Operand, // membermask
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        And, // .and
        Or, // .or
        Xor, // .xor
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ReduxSyncOpB32 {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub b32: (), // .b32
        pub dst: Operand, // dst
        pub src: Operand, // src
        pub membermask: Operand, // membermask
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ReduxSyncOpAbsNanF32 {
        pub sync: (), // .sync
        pub op: Op, // .op
        pub abs: bool, // {.abs}
        pub nan: bool, // {.NaN}
        pub f32: (), // .f32
        pub dst: Operand, // dst
        pub src: Operand, // src
        pub membermask: Operand, // membermask
    }

}
