use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `red{.sem}{.scope}{.space}.op{.level::cache_hint}.type [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16   [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2 [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16  [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2 [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32 [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type [a], b{, cache-policy};`
/// `red{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type [a], b{, cache-policy};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedOpcode {
    /// `red{.sem}{.scope}{.space}.op{.level::cache_hint}.type [a], b{, cache-policy};`
    Scalar(Scalar),
    /// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16   [a], b{, cache-policy};`
    /// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2 [a], b{, cache-policy};`
    /// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16  [a], b{, cache-policy};`
    /// `red{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2 [a], b{, cache-policy};`
    ScalarAddNoFtz(ScalarAddNoFtz),
    /// `red{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32 [a], b{, cache-policy};`
    VectorAdd32(VectorAdd32),
    /// `red{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type [a], b{, cache-policy};`
    VectorHalf(VectorHalf),
    /// `red{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type [a], b{, cache-policy};`
    VectorPacked(VectorPacked),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: ScalarOperation,
    pub cache_hint: Option<CacheHint>,
    pub data_type: ScalarType,
    pub address: AddressOperand,
    pub source: RegisterOperand,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarAddNoFtz {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub data_type: ScalarAddNoFtzType,
    pub address: AddressOperand,
    pub source: RegisterOperand,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorAdd32 {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<VectorStateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub address: AddressOperand,
    pub element_type: Vec32ElementType,
    pub source: Vec32Registers,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorHalf {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<VectorStateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub operation: VectorOperation,
    pub element_type: HalfWordType,
    pub address: AddressOperand,
    pub source: Vec16Registers,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorPacked {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<VectorStateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub operation: VectorOperation,
    pub element_type: PackedType,
    pub address: AddressOperand,
    pub source: Vec32Registers,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semantics {
    /// `.relaxed`
    Relaxed,
    /// `.release`
    Release,
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
pub enum StateSpace {
    /// `.global`
    Global,
    /// `.shared`
    Shared(SharedSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSpace {
    /// `.shared`
    Default,
    /// `.shared::cta`
    Cta,
    /// `.shared::cluster`
    Cluster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorStateSpace {
    /// `.global`
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheHint {
    /// `.L2::cache_hint`
    L2CacheHint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarOperation {
    /// `.and`
    And,
    /// `.or`
    Or,
    /// `.xor`
    Xor,
    /// `.add`
    Add,
    /// `.inc`
    Inc,
    /// `.dec`
    Dec,
    /// `.min`
    Min,
    /// `.max`
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
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
pub enum ScalarAddNoFtzType {
    /// `.f16`
    F16,
    /// `.f16x2`
    F16x2,
    /// `.bf16`
    Bf16,
    /// `.bf16x2`
    Bf16x2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec32ElementType {
    /// `.f32`
    F32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorOperation {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalfWordType {
    /// `.f16`
    F16,
    /// `.bf16`
    Bf16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackedType {
    /// `.f16x2`
    F16x2,
    /// `.bf16x2`
    Bf16x2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vec16Registers {
    /// `.vec_16_bit = .v2`
    V2([RegisterOperand; 2]),
    /// `.vec_16_bit = .v4`
    V4([RegisterOperand; 4]),
    /// `.vec_16_bit = .v8`
    V8([RegisterOperand; 8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vec32Registers {
    /// `.vec_32_bit = .v2`
    V2([RegisterOperand; 2]),
    /// `.vec_32_bit = .v4`
    V4([RegisterOperand; 4]),
}
