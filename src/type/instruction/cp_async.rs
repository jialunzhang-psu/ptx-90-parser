//! Original PTX specification:
//!
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, src-size}{, cache-policy};
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, src-size}{, cache-policy};
//! cp.async.ca.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
//! cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy} ;
//! .level::cache_hint =     { .L2::cache_hint };
//! .level::prefetch_size =  { .L2::64B, .L2::128B, .L2::256B };
//! cp-size = { 4, 8, 16 };
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        Shared, // .shared
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelPrefetchSize {
        L264b, // .L2::64B
        L2128b, // .L2::128B
        L2256b, // .L2::256B
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CpSize {
        _4, // 4
        _8, // 8
        _16, // 16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
        pub async_: (), // .async
        pub ca: (), // .ca
        pub state: State, // .state
        pub global: (), // .global
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand, // [dst]
        pub src: AddressOperand, // [src]
        pub cp_size: CpSize, // cp-size
        pub src_size: Option<Operand>, // {, src-size}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
        pub async_: (), // .async
        pub cg: (), // .cg
        pub state: State, // .state
        pub global: (), // .global
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand, // [dst]
        pub src: AddressOperand, // [src]
        pub imm_16: (), // 16
        pub src_size: Option<Operand>, // {, src-size}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
        pub async_: (), // .async
        pub ca: (), // .ca
        pub state: State, // .state
        pub global: (), // .global
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand, // [dst]
        pub src: AddressOperand, // [src]
        pub cp_size: CpSize, // cp-size
        pub ignore_src: Option<Operand>, // {, ignore-src}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
        pub async_: (), // .async
        pub cg: (), // .cg
        pub state: State, // .state
        pub global: (), // .global
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand, // [dst]
        pub src: AddressOperand, // [src]
        pub imm_16: (), // 16
        pub ignore_src: Option<Operand>, // {, ignore-src}
        pub cache_policy: Option<Operand>, // {, cache-policy}
    }

}
