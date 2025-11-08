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
        pub async_: (),                               // .async
        pub bulk: (),                                 // .bulk
        pub prefetch: (),                             // .prefetch
        pub l2: (),                                   // .L2
        pub src: Src,                                 // .src
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub srcmem: AddressOperand,                   // [srcMem]
        pub size: GeneralOperand,                     // size
        pub cache_policy: Option<GeneralOperand>,     // {, cache-policy}
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpAsyncBulkPrefetchL2SrcLevelCacheHint;
pub use section_0::LevelCacheHint as LevelCacheHint0;
pub use section_0::Src as Src0;
