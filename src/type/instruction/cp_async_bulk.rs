//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster;
//! cp.async.bulk.dst.src.completion_mechanism{.multicast}{.level::cache_hint} [dstMem], [srcMem], size, [mbar] {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> shared::cluster
//! cp.async.bulk.dst.src.completion_mechanism [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ----------------------------------------------------------------
//! // shared::cta -> global
//! cp.async.bulk.dst.src.completion_mechanism{.level::cache_hint}{.cp_mask} [dstMem], [srcMem], size {, cache-policy} {, byteMask};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint =    { .L2::cache_hint };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub mbar: AddressOperand, // [mbar]
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Multicast {
        MulticastCluster, // .multicast::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub multicast: Option<Multicast>, // {.multicast}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub mbar: AddressOperand, // [mbar]
        pub ctamask: Option<Operand>, // {, ctaMask}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}

pub mod section_2 {
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
    pub struct CpAsyncBulkDstSrcCompletionMechanism {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub mbar: AddressOperand, // [mbar]
    }

}

pub mod section_3 {
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
    pub struct CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub cp_mask: bool, // {.cp_mask}
        pub dstmem: AddressOperand, // [dstMem]
        pub srcmem: AddressOperand, // [srcMem]
        pub size: Operand, // size
        pub cache_policy: Option<Operand>, // {, cache-policy}
        pub bytemask: Option<Operand>, // {, byteMask}
    }

}
