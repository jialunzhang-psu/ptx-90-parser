//! Original PTX specification:
//!
//! applypriority{.global}.level::eviction_priority  [a], size;
//! .level::eviction_priority = { .L2::evict_normal };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum LevelEvictionPriority {
        L2EvictNormal, // .L2::evict_normal
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ApplypriorityGlobalLevelEvictionPriority {
        pub global: bool, // {.global}
        pub level_eviction_priority: LevelEvictionPriority, // .level::eviction_priority
        pub a: AddressOperand, // [a]
        pub size: Operand, // size
    }

}
