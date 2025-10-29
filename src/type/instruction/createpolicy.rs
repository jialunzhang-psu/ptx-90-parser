use crate::r#type::common::{AddressOperand, Immediate, Operand, RegisterOperand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Createpolicy {
    /// `createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64 cache-policy, [a], primary-size, total-size;`
    Range {
        /// `.global`
        global: bool,
        /// `.level::primary_priority`
        primary_priority: PrimaryPriority,
        /// `.level::secondary_priority`
        secondary_priority: Option<SecondaryPriority>,
        /// `cache-policy`
        destination: RegisterOperand,
        /// `[a]`
        address: AddressOperand,
        /// `primary-size`
        primary_size: Operand,
        /// `total-size`
        total_size: Operand,
    },
    /// `createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64 cache-policy{, fraction};`
    Fractional {
        /// `.level::primary_priority`
        primary_priority: PrimaryPriority,
        /// `.level::secondary_priority`
        secondary_priority: Option<SecondaryPriority>,
        /// `cache-policy`
        destination: RegisterOperand,
        /// `fraction`
        fraction: Option<Immediate>,
    },
    /// `createpolicy.cvt.L2.b64 cache-policy, access-property;`
    Convert {
        /// `.L2`
        level: Level,
        /// `cache-policy`
        destination: RegisterOperand,
        /// `access-property`
        access_property: RegisterOperand,
    },
}

/// `.level::primary_priority = { .L2::evict_last, .L2::evict_normal, .L2::evict_first, .L2::evict_unchanged };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryPriority {
    /// `.L2::evict_last`
    L2EvictLast,
    /// `.L2::evict_normal`
    L2EvictNormal,
    /// `.L2::evict_first`
    L2EvictFirst,
    /// `.L2::evict_unchanged`
    L2EvictUnchanged,
}

/// `.level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryPriority {
    /// `.L2::evict_first`
    L2EvictFirst,
    /// `.L2::evict_unchanged`
    L2EvictUnchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// `.L2`
    L2,
}
