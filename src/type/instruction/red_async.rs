//! Original PTX specification:
//!
//! // Increment and Decrement reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  =                 { .relaxed };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .op   =                 { .inc, .dec };
//! .type =                 { .u32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // MIN and MAX reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .min, .max };
//! .type = { .u32, .s32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // Bitwise AND, OR and XOR reductions
//! red.async.sem.scope{.ss}.completion_mechanism.op.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .op   = { .and, .or, .xor };
//! .type = { .b32 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ------------------------------------------------------------------
//! // ADD reductions
//! red.async.sem.scope{.ss}.completion_mechanism.add.type [a], b, [mbar];
//! .sem  = { .relaxed };
//! .scope = { .cluster };
//! .ss   = { .shared::cluster };
//! .type = { .u32, .s32, .u64 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------
//! // Alternate floating point type:
//! red.async{.mmio}.sem.scope{.ss}.add.type [a], b;
//! .sem  = { .release };
//! .scope = { .gpu, .cluster };
//! .ss   = { .global };
//! .type = { .u32, .s32, .u64, .s64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Inc, // .inc
        Dec, // .dec
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RedAsyncSemScopeSsCompletionMechanismOpType {
        pub async_: (), // .async
        pub sem: Sem, // .sem
        pub scope: Scope, // .scope
        pub ss: Option<Ss>, // {.ss}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub op: Op, // .op
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RedAsyncSemScopeSsCompletionMechanismOpType1 {
        pub async_: (), // .async
        pub sem: Sem, // .sem
        pub scope: Scope, // .scope
        pub ss: Option<Ss>, // {.ss}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub op: Op, // .op
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        And, // .and
        Xor, // .xor
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RedAsyncSemScopeSsCompletionMechanismOpType2 {
        pub async_: (), // .async
        pub sem: Sem, // .sem
        pub scope: Scope, // .scope
        pub ss: Option<Ss>, // {.ss}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub op: Op, // .op
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_3 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RedAsyncSemScopeSsCompletionMechanismAddType {
        pub async_: (), // .async
        pub sem: Sem, // .sem
        pub scope: Scope, // .scope
        pub ss: Option<Ss>, // {.ss}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub add: (), // .add
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_4 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Gpu, // .gpu
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RedAsyncMmioSemScopeSsAddType {
        pub async_: (), // .async
        pub mmio: bool, // {.mmio}
        pub sem: Sem, // .sem
        pub scope: Scope, // .scope
        pub ss: Option<Ss>, // {.ss}
        pub add: (), // .add
        pub type_: Type, // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
    }

}
