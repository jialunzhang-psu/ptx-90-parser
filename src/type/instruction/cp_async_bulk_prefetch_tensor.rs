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
        Im2colW128,  // .im2col::w::128
        TileGather4, // .tile::gather4
        Im2colW,     // .im2col::w
        Im2col,      // .im2col
        Tile,        // .tile
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint {
        pub async_: (),                               // .async
        pub bulk: (),                                 // .bulk
        pub prefetch: (),                             // .prefetch
        pub tensor: (),                               // .tensor
        pub dim: Dim,                                 // .dim
        pub l2: (),                                   // .L2
        pub src: Src,                                 // .src
        pub load_mode: Option<LoadMode>,              // {.load_mode}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub tensormap: TexHandler2,                   // [tensorMap, tensorCoords]
        pub im2colinfo: Option<GeneralOperand>,       // {, im2colInfo}
        pub cache_policy: Option<GeneralOperand>,     // {, cache-policy}
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint;
pub use section_0::Dim as Dim0;
pub use section_0::LevelCacheHint as LevelCacheHint0;
pub use section_0::LoadMode as LoadMode0;
pub use section_0::Src as Src0;
