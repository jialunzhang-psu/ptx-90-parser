use crate::r#type::common::AddressOperand;

/// `applypriority{.global}.level::eviction_priority [a], size;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applypriority {
    /// `.global`
    pub global: bool,
    /// `.L2::evict_normal`
    pub eviction_priority: EvictionPriority,
    /// `[a]`
    pub address: AddressOperand,
    /// `size`
    pub size: Size,
}

/// `.level::eviction_priority = { .L2::evict_normal };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPriority {
    /// `.L2::evict_normal`
    L2EvictNormal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// `128`
    B128,
}
