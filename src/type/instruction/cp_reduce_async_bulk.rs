//! Original PTX specification:
//!
//! cp.reduce.async.bulk.dst.src.completion_mechanism.redOp.type [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .b32, .u32, .s32, .b64, .u64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.redOp.type [dstMem], [srcMem], size{, cache-policy};
//! .dst =                  { .global      };
//! .src =                  { .shared::cta };
//! ----------------------------------------------------------------
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint    = { .L2::cache_hint };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .f16, .bf16, .b32, .u32, .s32, .b64, .u64, .s64, .f32, .f64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.add.noftz.type [dstMem], [srcMem], size{, cache-policy};
//! .dst  =                 { .global };
//! .src  =                 { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .type =                 { .f16, .bf16 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redop {
        And, // .and
        Or, // .or
        Xor, // .xor
        Add, // .add
        Inc, // .inc
        Dec, // .dec
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        U32, // .u32
        S32, // .s32
        B64, // .b64
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
        pub reduce: (), // .reduce
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub redop: Redop, // .redOp
        pub type_: Type, // .type
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redop {
        And, // .and
        Or, // .or
        Xor, // .xor
        Add, // .add
        Inc, // .inc
        Dec, // .dec
        Min, // .min
        Max, // .max
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        U32, // .u32
        S32, // .s32
        B64, // .b64
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
        pub reduce: (), // .reduce
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: bool, // {.level::cache_hint}
        pub redop: Redop, // .redOp
        pub type_: Type, // .type
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        BulkGroup, // .bulk_group
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F16, // .f16
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
        pub reduce: (), // .reduce
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub add: (), // .add
        pub noftz: (), // .noftz
        pub type_: Type, // .type
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}
