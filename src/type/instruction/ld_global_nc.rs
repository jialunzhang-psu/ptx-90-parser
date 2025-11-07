//! Original PTX specification:
//!
//! ld.global{.cop}.nc{.level::cache_hint}{.level::prefetch_size}.type                 d, [a]{, cache-policy};
//! ld.global{.cop}.nc{.level::cache_hint}{.level::prefetch_size}.vec.type             d, [a]{, cache-policy};
//! ld.global.nc{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}.type      d, [a]{, cache-policy};
//! ld.global.nc{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}.vec.type  d, [a]{, cache-policy};
//! .cop  =                     { .ca, .cg, .cs };     // cache operation
//! .level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged, .L1::evict_first, .L1::evict_last, .L1::no_allocate};
//! .level2::eviction_priority = {.L2::evict_normal, .L2::evict_first, .L2::evict_last};
//! .level::cache_hint =        { .L2::cache_hint };
//! .level::prefetch_size =     { .L2::64B, .L2::128B, .L2::256B };
//! .vec  =                     { .v2, .v4, .v8 };
//! .type =                     { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cop {
        Ca, // .ca
        Cg, // .cg
        Cs, // .cs
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelPrefetchSize {
        L2128b, // .L2::128B
        L2256b, // .L2::256B
        L264b, // .L2::64B
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B128, // .b128
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
        B8, // .b8
        U8, // .u8
        S8, // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
        V8, // .v8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level1EvictionPriority {
        L1EvictUnchanged, // .L1::evict_unchanged
        L1EvictNormal, // .L1::evict_normal
        L1EvictFirst, // .L1::evict_first
        L1NoAllocate, // .L1::no_allocate
        L1EvictLast, // .L1::evict_last
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level2EvictionPriority {
        L2EvictNormal, // .L2::evict_normal
        L2EvictFirst, // .L2::evict_first
        L2EvictLast, // .L2::evict_last
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType {
        pub global: (), // .global
        pub cop: Option<Cop>, // {.cop}
        pub nc: (), // .nc
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType {
        pub global: (), // .global
        pub cop: Option<Cop>, // {.cop}
        pub nc: (), // .nc
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub vec: Vec, // .vec
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType {
        pub global: (), // .global
        pub nc: (), // .nc
        pub level1_eviction_priority: Option<Level1EvictionPriority>, // {.level1::eviction_priority}
        pub level2_eviction_priority: Option<Level2EvictionPriority>, // {.level2::eviction_priority}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType {
        pub global: (), // .global
        pub nc: (), // .nc
        pub level1_eviction_priority: Option<Level1EvictionPriority>, // {.level1::eviction_priority}
        pub level2_eviction_priority: Option<Level2EvictionPriority>, // {.level2::eviction_priority}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub vec: Vec, // .vec
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub cache_policy: Option<GeneralOperand>, // {, cache-policy}
    }

}
