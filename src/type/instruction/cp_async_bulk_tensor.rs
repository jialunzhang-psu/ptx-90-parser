//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.multicast}{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> global;
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {, cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .tile::scatter4, .im2col_no_offs };
//! .level::cache_hint =    { .L2::cache_hint };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dim {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
        _4d, // .4d
        _5d, // .5d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoadMode {
        Im2colW128, // .im2col::w::128
        TileGather4, // .tile::gather4
        Im2colW, // .im2col::w
        Im2col, // .im2col
        Tile, // .tile
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub tensor: (), // .tensor
        pub dim: Dim, // .dim
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub load_mode: Option<LoadMode>, // {.load_mode}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub cta_group: Option<CtaGroup>, // {.cta_group}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub dstmem: AddressOperand, // [dstMem]
        pub tensormap: TexHandler2, // [tensorMap, tensorCoords]
        pub mbar: AddressOperand, // [mbar]
        pub im2colinfo: Option<GeneralOperand>, // {, im2colInfo}
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dim {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
        _4d, // .4d
        _5d, // .5d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoadMode {
        Im2colW128, // .im2col::w::128
        TileGather4, // .tile::gather4
        Im2colW, // .im2col::w
        Im2col, // .im2col
        Tile, // .tile
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
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub tensor: (), // .tensor
        pub dim: Dim, // .dim
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub load_mode: Option<LoadMode>, // {.load_mode}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub multicast: Option<Multicast>, // {.multicast}
        pub cta_group: Option<CtaGroup>, // {.cta_group}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub dstmem: AddressOperand, // [dstMem]
        pub tensormap: TexHandler2, // [tensorMap, tensorCoords]
        pub mbar: AddressOperand, // [mbar]
        pub im2colinfo: Option<GeneralOperand>, // {, im2colInfo}
        pub ctamask: Option<GeneralOperand>, // {, ctaMask}
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dim {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
        _4d, // .4d
        _5d, // .5d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dst {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoadMode {
        TileScatter4, // .tile::scatter4
        Im2colNoOffs, // .im2col_no_offs
        Tile, // .tile
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
    pub struct CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub tensor: (), // .tensor
        pub dim: Dim, // .dim
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub load_mode: Option<LoadMode>, // {.load_mode}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub tensormap: TexHandler2, // [tensorMap, tensorCoords]
        pub srcmem: AddressOperand, // [srcMem]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}
