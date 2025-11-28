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
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Inc, // .inc
        Dec, // .dec
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
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
        pub span: Span,
    }

}

pub mod section_1 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
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
        pub span: Span,
    }

}

pub mod section_2 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        And, // .and
        Xor, // .xor
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        B32, // .b32
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
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
        pub span: Span,
    }

}

pub mod section_3 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
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
        pub span: Span,
    }

}

pub mod section_4 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
        Gpu, // .gpu
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
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
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::RedAsyncSemScopeSsCompletionMechanismOpType;
pub use section_0::Sem as Sem0;
pub use section_0::Scope as Scope0;
pub use section_0::Ss as Ss0;
pub use section_0::CompletionMechanism as CompletionMechanism0;
pub use section_0::Op as Op0;
pub use section_0::Type as Type0;
pub use section_1::RedAsyncSemScopeSsCompletionMechanismOpType1;
pub use section_1::Sem as Sem1;
pub use section_1::Scope as Scope1;
pub use section_1::Ss as Ss1;
pub use section_1::CompletionMechanism as CompletionMechanism1;
pub use section_1::Op as Op1;
pub use section_1::Type as Type1;
pub use section_2::RedAsyncSemScopeSsCompletionMechanismOpType2;
pub use section_2::Sem as Sem2;
pub use section_2::Scope as Scope2;
pub use section_2::Ss as Ss2;
pub use section_2::CompletionMechanism as CompletionMechanism2;
pub use section_2::Op as Op2;
pub use section_2::Type as Type2;
pub use section_3::RedAsyncSemScopeSsCompletionMechanismAddType;
pub use section_3::Sem as Sem3;
pub use section_3::Scope as Scope3;
pub use section_3::Ss as Ss3;
pub use section_3::CompletionMechanism as CompletionMechanism3;
pub use section_3::Type as Type3;
pub use section_4::RedAsyncMmioSemScopeSsAddType;
pub use section_4::Sem as Sem4;
pub use section_4::Scope as Scope4;
pub use section_4::Ss as Ss4;
pub use section_4::Type as Type4;
