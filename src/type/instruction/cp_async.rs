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
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum State {
        SharedCta, // .shared::cta
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum LevelPrefetchSize {
        L2128b, // .L2::128B
        L2256b, // .L2::256B
        L264b,  // .L2::64B
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CpSize {
        _16, // 16
        _4,  // 4
        _8,  // 8
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize {
        pub async_: (),                                     // .async
        pub ca: (),                                         // .ca
        pub state: State,                                   // .state
        pub global: (),                                     // .global
        pub level_cache_hint: Option<LevelCacheHint>,       // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand,                            // [dst]
        pub src: AddressOperand,                            // [src]
        pub cp_size: CpSize,                                // cp-size
        pub src_size: Option<GeneralOperand>,               // {, src-size}
        pub cache_policy: Option<GeneralOperand>,           // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize {
        pub async_: (),                                     // .async
        pub cg: (),                                         // .cg
        pub state: State,                                   // .state
        pub global: (),                                     // .global
        pub level_cache_hint: Option<LevelCacheHint>,       // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand,                            // [dst]
        pub src: AddressOperand,                            // [src]
        pub imm_16: (),                                     // 16
        pub src_size: Option<GeneralOperand>,               // {, src-size}
        pub cache_policy: Option<GeneralOperand>,           // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 {
        pub async_: (),                                     // .async
        pub ca: (),                                         // .ca
        pub state: State,                                   // .state
        pub global: (),                                     // .global
        pub level_cache_hint: Option<LevelCacheHint>,       // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand,                            // [dst]
        pub src: AddressOperand,                            // [src]
        pub cp_size: CpSize,                                // cp-size
        pub ignore_src: Option<GeneralOperand>,             // {, ignore-src}
        pub cache_policy: Option<GeneralOperand>,           // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 {
        pub async_: (),                                     // .async
        pub cg: (),                                         // .cg
        pub state: State,                                   // .state
        pub global: (),                                     // .global
        pub level_cache_hint: Option<LevelCacheHint>,       // {.level::cache_hint}
        pub level_prefetch_size: Option<LevelPrefetchSize>, // {.level::prefetch_size}
        pub dst: AddressOperand,                            // [dst]
        pub src: AddressOperand,                            // [src]
        pub imm_16: (),                                     // 16
        pub ignore_src: Option<GeneralOperand>,             // {, ignore-src}
        pub cache_policy: Option<GeneralOperand>,           // {, cache-policy}
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize;
pub use section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1;
pub use section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize;
pub use section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1;
pub use section_0::CpSize as CpSize0;
pub use section_0::LevelCacheHint as LevelCacheHint0;
pub use section_0::LevelPrefetchSize as LevelPrefetchSize0;
pub use section_0::State as State0;
