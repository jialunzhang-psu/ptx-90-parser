use crate::r#type::common::AddressOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefetch {
    /// `prefetch{.space}.level [a];`
    DataCache {
        space: Option<Space>,
        level: Level,
        address: AddressOperand,
    },
    /// `prefetch.global.level::eviction_priority [a];`
    GlobalEviction {
        eviction: Eviction,
        address: AddressOperand,
    },
    /// `prefetchu.L1 [a];`
    Uniform { address: AddressOperand },
    /// `prefetch{.tensormap_space}.tensormap [a];`
    TensorMap {
        space: Option<TensorMapSpace>,
        address: AddressOperand,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// `.global`
    Global,
    /// `.local`
    Local,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// `.L1`
    L1,
    /// `.L2`
    L2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eviction {
    /// `.L2::evict_last`
    L2EvictLast,
    /// `.L2::evict_normal`
    L2EvictNormal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorMapSpace {
    /// `.const`
    Const,
    /// `.param`
    Param,
}
