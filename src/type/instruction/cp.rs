use crate::r#type::common::{AddressOperand, Operand, PredicateRegister, RegisterOperand};

/// Type-safe representation of every `cp` syntax variant described in the cache specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpOpcode {
    /// `cp.async.ca.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, src-size}{, cache-policy};`
    AsyncCaWithSrcSize(CpAsyncCaWithSrcSize),
    /// `cp.async.cg.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, src-size}{, cache-policy};`
    AsyncCgWithSrcSize(CpAsyncCgWithSrcSize),
    /// `cp.async.ca.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], cp-size{, ignore-src}{, cache-policy};`
    AsyncCaIgnoreSrc(CpAsyncCaIgnoreSrc),
    /// `cp.async.cg.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy};`
    AsyncCgIgnoreSrc(CpAsyncCgIgnoreSrc),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpAsyncCaWithSrcSize {
    pub shared_space: CpSharedSpace,
    pub cache_hint: Option<CpCacheHint>,
    pub prefetch_size: Option<CpPrefetchSize>,
    pub destination: AddressOperand,
    pub source: AddressOperand,
    pub copy_size: CpCopySize,
    pub source_size: Option<Operand>,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpAsyncCgWithSrcSize {
    pub shared_space: CpSharedSpace,
    pub cache_hint: Option<CpCacheHint>,
    pub prefetch_size: Option<CpPrefetchSize>,
    pub destination: AddressOperand,
    pub source: AddressOperand,
    pub source_size: Option<Operand>,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpAsyncCaIgnoreSrc {
    pub shared_space: CpSharedSpace,
    pub cache_hint: Option<CpCacheHint>,
    pub prefetch_size: Option<CpPrefetchSize>,
    pub destination: AddressOperand,
    pub source: AddressOperand,
    pub copy_size: CpCopySize,
    pub ignore_src: Option<PredicateRegister>,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpAsyncCgIgnoreSrc {
    pub shared_space: CpSharedSpace,
    pub cache_hint: Option<CpCacheHint>,
    pub prefetch_size: Option<CpPrefetchSize>,
    pub destination: AddressOperand,
    pub source: AddressOperand,
    pub ignore_src: Option<PredicateRegister>,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpSharedSpace {
    /// `.shared`
    Default,
    /// `.shared::cta`
    Cta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpCacheHint {
    /// `.L2::cache_hint`
    L2CacheHint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpPrefetchSize {
    /// `.L2::64B`
    L264B,
    /// `.L2::128B`
    L2128B,
    /// `.L2::256B`
    L2256B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpCopySize {
    /// `4`
    Bytes4,
    /// `8`
    Bytes8,
    /// `16`
    Bytes16,
}
