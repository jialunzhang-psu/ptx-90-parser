//! Original PTX specification:
//!
//! cp.async.bulk.prefetch.L2.src{.level::cache_hint}   [srcMem], size {, cache-policy};
//! .src =                { .global };
//! .level::cache_hint =  { .L2::cache_hint };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Src {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkPrefetchL2SrcLevelCacheHint {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub prefetch: (), // .prefetch
        pub l2: (), // .L2
        pub src: Src, // .src
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub srcmem: AddressOperand, // [srcMem]
        pub size: GeneralOperand, // size
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}
