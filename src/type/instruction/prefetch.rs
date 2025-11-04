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
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        Global, // .global
        Local, // .local
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level {
        L1, // .L1
        L2, // .L2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelEvictionPriority {
        L2EvictLast, // .L2::evict_last
        L2EvictNormal, // .L2::evict_normal
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum TensormapSpace {
        Const, // .const
        Param, // .param
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrefetchSpaceLevel {
        pub space: Option<Space>, // {.space}
        pub level: Level, // .level
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrefetchGlobalLevelEvictionPriority {
        pub global: (), // .global
        pub level_eviction_priority: LevelEvictionPriority, // .level::eviction_priority
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrefetchuL1 {
        pub l1: (), // .L1
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PrefetchTensormapSpaceTensormap {
        pub tensormap_space: Option<TensormapSpace>, // {.tensormap_space}
        pub tensormap: (), // .tensormap
        pub a: AddressOperand, // [a]
    }

}
