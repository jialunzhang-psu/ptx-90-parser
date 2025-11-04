//! Original PTX specification:
//!
//! // global -> shared::cluster:
//! cp.async.bulk.prefetch.tensor.dim.L2.src{.load_mode}{.level::cache_hint} [tensorMap, tensorCoords] {, im2colInfo } {, cache-policy};
//! .src =                { .global };
//! .dim =                { .1d, .2d, .3d, .4d, .5d };
//! .load_mode =          { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =  { .L2::cache_hint };

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
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoadMode {
        Tile, // .tile
        TileGather4, // .tile::gather4
        Im2col, // .im2col
        Im2colW, // .im2col::w
        Im2colW128, // .im2col::w::128
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub prefetch: (), // .prefetch
        pub tensor: (), // .tensor
        pub dim: Dim, // .dim
        pub l2: (), // .L2
        pub src: Src, // .src
        pub load_mode: Option<LoadMode>, // {.load_mode}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub tensormap: (Operand, Operand), // [tensorMap, tensorCoords]
        pub im2colinfo: Option<Operand>, // {, im2colInfo}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}
