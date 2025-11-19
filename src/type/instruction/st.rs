//! Original PTX specification:
//!
//! st{.weak}{.ss}{.cop}{.level::cache_hint}{.vec}.type   [a], b{, cache-policy};
//! st{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.volatile{.ss}{.vec}.type                           [a], b;
//! st.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.release.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};
//! st.mmio.relaxed.sys{.global}.type         [a], b;
//! .ss =                       { .global, .local, .param, .param::func, .shared, .shared::cta, .shared::cluster};
//! .level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged,
//! .L1::evict_first, .L1::evict_last, .L1::no_allocate };
//! .level2::eviction_priority = { .L2::evict_normal, .L2::evict_first, .L2::evict_last };
//! .level::cache_hint =        { .L2::cache_hint };
//! .cop =                      { .wb, .cg, .cs, .wt };
//! .sem =                      { .relaxed, .release };
//! .scope =                    { .cta, .cluster, .gpu, .sys };
//! .vec =                      { .v2, .v4, .v8 };
//! .type =                     { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
        ParamFunc,     // .param::func
        SharedCta,     // .shared::cta
        Global,        // .global
        Shared,        // .shared
        Local,         // .local
        Param,         // .param
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cop {
        Wb, // .wb
        Cg, // .cg
        Cs, // .cs
        Wt, // .wt
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelCacheHint {
        L2CacheHint, // .L2::cache_hint
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
        V8, // .v8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B128, // .b128
        B16,  // .b16
        B32,  // .b32
        B64,  // .b64
        U16,  // .u16
        U32,  // .u32
        U64,  // .u64
        S16,  // .s16
        S32,  // .s32
        S64,  // .s64
        F32,  // .f32
        F64,  // .f64
        B8,   // .b8
        U8,   // .u8
        S8,   // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level1EvictionPriority {
        L1EvictUnchanged, // .L1::evict_unchanged
        L1EvictNormal,    // .L1::evict_normal
        L1EvictFirst,     // .L1::evict_first
        L1NoAllocate,     // .L1::no_allocate
        L1EvictLast,      // .L1::evict_last
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level2EvictionPriority {
        L2EvictNormal, // .L2::evict_normal
        L2EvictFirst,  // .L2::evict_first
        L2EvictLast,   // .L2::evict_last
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
        Gpu,     // .gpu
        Sys,     // .sys
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StWeakSsCopLevelCacheHintVecType {
        pub weak: bool,                               // {.weak}
        pub ss: Option<Ss>,                           // {.ss}
        pub cop: Option<Cop>,                         // {.cop}
        pub level_cache_hint: Option<LevelCacheHint>, // {.level::cache_hint}
        pub vec: Option<Vec>,                         // {.vec}
        pub type_: Type,                              // .type
        pub a: AddressOperand,                        // [a]
        pub b: GeneralOperand,                        // b
        pub cache_policy: Option<GeneralOperand>,     // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        pub weak: bool,                                               // {.weak}
        pub ss: Option<Ss>,                                           // {.ss}
        pub level1_eviction_priority: Option<Level1EvictionPriority>, // {.level1::eviction_priority}
        pub level2_eviction_priority: Option<Level2EvictionPriority>, // {.level2::eviction_priority}
        pub level_cache_hint: Option<LevelCacheHint>,                 // {.level::cache_hint}
        pub vec: Option<Vec>,                                         // {.vec}
        pub type_: Type,                                              // .type
        pub a: AddressOperand,                                        // [a]
        pub b: GeneralOperand,                                        // b
        pub cache_policy: Option<GeneralOperand>,                     // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StVolatileSsVecType {
        pub volatile: (),      // .volatile
        pub ss: Option<Ss>,    // {.ss}
        pub vec: Option<Vec>,  // {.vec}
        pub type_: Type,       // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        pub relaxed: (),                                              // .relaxed
        pub scope: Scope,                                             // .scope
        pub ss: Option<Ss>,                                           // {.ss}
        pub level1_eviction_priority: Option<Level1EvictionPriority>, // {.level1::eviction_priority}
        pub level2_eviction_priority: Option<Level2EvictionPriority>, // {.level2::eviction_priority}
        pub level_cache_hint: Option<LevelCacheHint>,                 // {.level::cache_hint}
        pub vec: Option<Vec>,                                         // {.vec}
        pub type_: Type,                                              // .type
        pub a: AddressOperand,                                        // [a]
        pub b: GeneralOperand,                                        // b
        pub cache_policy: Option<GeneralOperand>,                     // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType {
        pub release: (),                                              // .release
        pub scope: Scope,                                             // .scope
        pub ss: Option<Ss>,                                           // {.ss}
        pub level1_eviction_priority: Option<Level1EvictionPriority>, // {.level1::eviction_priority}
        pub level2_eviction_priority: Option<Level2EvictionPriority>, // {.level2::eviction_priority}
        pub level_cache_hint: Option<LevelCacheHint>,                 // {.level::cache_hint}
        pub vec: Option<Vec>,                                         // {.vec}
        pub type_: Type,                                              // .type
        pub a: AddressOperand,                                        // [a]
        pub b: GeneralOperand,                                        // b
        pub cache_policy: Option<GeneralOperand>,                     // {, cache-policy}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct StMmioRelaxedSysGlobalType {
        pub mmio: (),          // .mmio
        pub relaxed: (),       // .relaxed
        pub sys: (),           // .sys
        pub global: bool,      // {.global}
        pub type_: Type,       // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Cop as Cop0;
pub use section_0::Level1EvictionPriority as Level1EvictionPriority0;
pub use section_0::Level2EvictionPriority as Level2EvictionPriority0;
pub use section_0::LevelCacheHint as LevelCacheHint0;
pub use section_0::Scope as Scope0;
pub use section_0::Ss as Ss0;
pub use section_0::StMmioRelaxedSysGlobalType;
pub use section_0::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType;
pub use section_0::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType;
pub use section_0::StVolatileSsVecType;
pub use section_0::StWeakSsCopLevelCacheHintVecType;
pub use section_0::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType;
pub use section_0::Type as Type0;
pub use section_0::Vec as Vec0;
