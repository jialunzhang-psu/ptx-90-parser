//! Original PTX specification:
//!
//! prefetch{.space}.level                    [a];   // prefetch to data cache
//! prefetch.global.level::eviction_priority  [a];   // prefetch to data cache
//! prefetchu.L1  [a];             // prefetch to uniform cache
//! prefetch{.tensormap_space}.tensormap [a];  // prefetch the tensormap
//! .space =                    { .global, .local };
//! .level =                    { .L1, .L2 };
//! .level::eviction_priority = { .L2::evict_last, .L2::evict_normal };
//! .tensormap_space =          { .const, .param };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Space {
        Global, // .global
        Local,  // .local
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Level {
        L1, // .L1
        L2, // .L2
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum LevelEvictionPriority {
        L2EvictNormal, // .L2::evict_normal
        L2EvictLast,   // .L2::evict_last
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum TensormapSpace {
        Const, // .const
        Param, // .param
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct PrefetchSpaceLevel {
        pub space: Option<Space>, // {.space}
        pub level: Level,         // .level
        pub a: AddressOperand,    // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct PrefetchGlobalLevelEvictionPriority {
        pub global: (),                                     // .global
        pub level_eviction_priority: LevelEvictionPriority, // .level::eviction_priority
        pub a: AddressOperand,                              // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct PrefetchuL1 {
        pub l1: (),            // .L1
        pub a: AddressOperand, // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct PrefetchTensormapSpaceTensormap {
        pub tensormap_space: Option<TensormapSpace>, // {.tensormap_space}
        pub tensormap: (),                           // .tensormap
        pub a: AddressOperand,                       // [a]
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Level as Level0;
pub use section_0::LevelEvictionPriority as LevelEvictionPriority0;
pub use section_0::PrefetchGlobalLevelEvictionPriority;
pub use section_0::PrefetchSpaceLevel;
pub use section_0::PrefetchTensormapSpaceTensormap;
pub use section_0::PrefetchuL1;
pub use section_0::Space as Space0;
pub use section_0::TensormapSpace as TensormapSpace0;
