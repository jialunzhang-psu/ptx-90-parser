use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `st{.weak}{.ss}{.cop}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
/// `st{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
/// `st.volatile{.ss}{.vec}.type [a], b;`
/// `st.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
/// `st.release.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
/// `st.mmio.relaxed.sys{.global}.type [a], b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum St {
    /// `st{.weak}{.ss}{.cop}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
    Generic(Generic),
    /// `st{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
    Eviction(Eviction),
    /// `st.volatile{.ss}{.vec}.type [a], b;`
    Volatile(Volatile),
    /// `st.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
    Relaxed(Relaxed),
    /// `st.release.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache-policy};`
    Release(Release),
    /// `st.mmio.relaxed.sys{.global}.type [a], b;`
    Mmio(Mmio),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generic {
    pub weak: bool,
    pub state_space: Option<StateSpace>,
    pub cache_operator: Option<CacheOperator>,
    pub cache_hint: Option<CacheHint>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eviction {
    pub weak: bool,
    pub state_space: Option<StateSpace>,
    pub level1: Option<Level1EvictionPriority>,
    pub level2: Option<Level2EvictionPriority>,
    pub cache_hint: Option<CacheHint>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volatile {
    pub state_space: Option<ScopedStateSpace>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relaxed {
    pub scope: Scope,
    pub state_space: Option<ScopedStateSpace>,
    pub level1: Option<Level1EvictionPriority>,
    pub level2: Option<Level2EvictionPriority>,
    pub cache_hint: Option<CacheHint>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub scope: Scope,
    pub state_space: Option<ScopedStateSpace>,
    pub level1: Option<Level1EvictionPriority>,
    pub level2: Option<Level2EvictionPriority>,
    pub cache_hint: Option<CacheHint>,
    pub vector: Option<Vector>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mmio {
    pub state_space: Option<MmioStateSpace>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// `b`
    Register(RegisterOperand),
    /// `{ %reg, _, ... }`
    Vector(VectorElements),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorElements {
    /// `.v2`
    V2([VectorElement; 2]),
    /// `.v4`
    V4([VectorElement; 4]),
    /// `.v8`
    V8([VectorElement; 8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorElement {
    /// `%reg`
    Register(RegisterOperand),
    /// `_`
    Sink,
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
pub enum CacheOperator {
    /// `.wb`
    Wb,
    /// `.cg`
    Cg,
    /// `.cs`
    Cs,
    /// `.wt`
    Wt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheHint {
    /// `.L2::cache_hint`
    L2CacheHint,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.global`
    Global,
    /// `.local`
    Local,
    /// `.param{::func}`
    Param(ParamState),
    /// `.shared{::cta, ::cluster}`
    Shared(SharedState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamState {
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
pub enum MmioStateSpace {
    /// `.global`
    Global,
}
