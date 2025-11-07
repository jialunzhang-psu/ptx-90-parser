//! Original PTX specification:
//!
//! // shared::cta -> global
//! cp.reduce.async.bulk.tensor.dim.dst.src.redOp{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {,cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .im2col_no_offs };
//! .redOp =                { .add, .min, .max, .inc, .dec, .and, .or, .xor};

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
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Redop {
        Add, // .add
        Min, // .min
        Max, // .max
        Inc, // .inc
        Dec, // .dec
        And, // .and
        Xor, // .xor
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoadMode {
        Im2colNoOffs, // .im2col_no_offs
        Tile, // .tile
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        BulkGroup, // .bulk_group
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint {
        pub reduce: (), // .reduce
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub tensor: (), // .tensor
        pub dim: Dim, // .dim
        pub dst: Dst, // .dst
        pub src: Src, // .src
        pub redop: Redop, // .redOp
        pub load_mode: Option<LoadMode>, // {.load_mode}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub level_cache_hint: bool, // {.level::cache_hint}
        pub tensormap: TexHandler2, // [tensorMap, tensorCoords]
        pub srcmem: AddressOperand, // [srcMem]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}
