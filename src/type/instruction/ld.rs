use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `ld{.weak}{.ss}{.cop}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{.unified}{, cache-policy};`
/// `ld{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{.unified}{, cache-policy};`
/// `ld.volatile{.ss}{.level::prefetch_size}{.vec}.type d, [a];`
/// `ld.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{, cache-policy};`
/// `ld.acquire.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{, cache-policy};`
/// `ld.mmio.relaxed.sys{.global}.type d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ld {
    /// `ld{.weak}{.ss}{.cop}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{.unified}{, cache-policy};`
    Generic(Generic),
    /// `ld{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{.unified}{, cache-policy};`
    Eviction(Eviction),
    /// `ld.volatile{.ss}{.level::prefetch_size}{.vec}.type d, [a];`
    Volatile(Volatile),
    /// `ld.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{, cache-policy};`
    Relaxed(Scoped),
    /// `ld.acquire.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type d, [a]{, cache-policy};`
    Acquire(Scoped),
    /// `ld.mmio.relaxed.sys{.global}.type d, [a];`
    Mmio(Mmio),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generic {
    pub weak: bool,
    pub state_space: Option<StateSpace>,
    pub cache_operator: Option<CacheOperator>,
    pub cache_hint: Option<CacheHint>,
    pub prefetch_size: Option<PrefetchSize>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub destination: Destination,
    pub address: AddressOperand,
    pub unified: bool,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eviction {
    pub weak: bool,
    pub state_space: Option<StateSpace>,
    pub level1: Option<Level1EvictionPriority>,
    pub level2: Option<Level2EvictionPriority>,
    pub cache_hint: Option<CacheHint>,
    pub prefetch_size: Option<PrefetchSize>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub destination: Destination,
    pub address: AddressOperand,
    pub unified: bool,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volatile {
    pub state_space: Option<ScopedStateSpace>,
    pub prefetch_size: Option<PrefetchSize>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub destination: Destination,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scoped {
    pub scope: Scope,
    pub state_space: Option<ScopedStateSpace>,
    pub level1: Option<Level1EvictionPriority>,
    pub level2: Option<Level2EvictionPriority>,
    pub cache_hint: Option<CacheHint>,
    pub prefetch_size: Option<PrefetchSize>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub destination: Destination,
    pub address: AddressOperand,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mmio {
    pub state_space: Option<MmioStateSpace>,
    pub data_type: DataType,
    pub destination: Destination,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// `.cta`
    Cta,
    /// `.cluster`
    Cluster,
    /// `.gpu`
    Gpu,
    /// `.sys`
    Sys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector {
    /// `.v2`
    V2,
    /// `.v4`
    V4,
    /// `.v8`
    V8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b8`
    B8,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
    /// `.b128`
    B128,
    /// `.u8`
    U8,
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s8`
    S8,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.const`
    Const,
    /// `.global`
    Global,
    /// `.local`
    Local,
    /// `.param{::entry, ::func}`
    Param(ParamState),
    /// `.shared{::cta, ::cluster}`
    Shared(SharedState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamState {
    /// `.param::entry`
    Entry,
    /// `.param::func`
    Func,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedState {
    /// `.shared::cta`
    Cta,
    /// `.shared::cluster`
    Cluster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopedStateSpace {
    /// `.global`
    Global,
    /// `.shared{::cta, ::cluster}`
    Shared(SharedState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmioStateSpace {
    /// `.global`
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheHint {
    /// `.L2::cache_hint`
    L2CacheHint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchSize {
    /// `.L2::64B`
    L264B,
    /// `.L2::128B`
    L2128B,
    /// `.L2::256B`
    L2256B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level1EvictionPriority {
    /// `.L1::evict_normal`
    EvictNormal,
    /// `.L1::evict_unchanged`
    EvictUnchanged,
    /// `.L1::evict_first`
    EvictFirst,
    /// `.L1::evict_last`
    EvictLast,
    /// `.L1::no_allocate`
    NoAllocate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level2EvictionPriority {
    /// `.L2::evict_normal`
    EvictNormal,
    /// `.L2::evict_first`
    EvictFirst,
    /// `.L2::evict_last`
    EvictLast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    /// `d`
    Scalar(RegisterOperand),
    /// `{ %reg, _, ... }`
    Vector(DestinationElements),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestinationElements {
    /// `.v2`
    V2([DestinationElement; 2]),
    /// `.v4`
    V4([DestinationElement; 4]),
    /// `.v8`
    V8([DestinationElement; 8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestinationElement {
    /// `%reg`
    Register(RegisterOperand),
    /// `_`
    Sink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheOperator {
    /// `.ca`
    Ca,
    /// `.cg`
    Cg,
    /// `.cs`
    Cs,
    /// `.lu`
    Lu,
    /// `.cv`
    Cv,
}
