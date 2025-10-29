use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.space}.op.type d, [a], b, c;`
/// `atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;`
/// `atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;`
/// `atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32                 d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type d, [a], b{, cache-policy};`
/// `atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type    d, [a], b{, cache-policy};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    /// `atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};`
    Scalar(Scalar),
    /// `atom{.sem}{.scope}{.space}.op.type d, [a], b, c;`
    /// `atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;`
    /// `atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;`
    CompareSwap(CompareSwap),
    /// `atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b{, cache-policy};`
    Exchange128(Exchange128),
    /// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};`
    /// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};`
    /// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};`
    /// `atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};`
    AddNoFtz(AddNoFtz),
    /// `atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32 d, [a], b{, cache-policy};`
    VectorAdd32(VectorAdd32),
    /// `atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type d, [a], b{, cache-policy};`
    VectorHalf(VectorHalf),
    /// `atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type    d, [a], b{, cache-policy};`
    VectorPacked(VectorPacked),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: ScalarOperation,
    pub cache_hint: Option<CacheHint>,
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub address: AddressOperand,
    pub source: RegisterOperand,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareSwap {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub variant: CompareSwapVariant,
    pub destination: RegisterOperand,
    pub address: AddressOperand,
    pub compare: RegisterOperand,
    pub new_value: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exchange128 {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub destination: RegisterOperand,
    pub address: AddressOperand,
    pub source: RegisterOperand,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddNoFtz {
    pub semantics: Option<Semantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub cache_hint: Option<CacheHint>,
    pub data_type: NoFtzType,
    pub destination: RegisterOperand,
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
    pub destination: Vec32Registers,
    pub address: AddressOperand,
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
    pub destination: Vec16Registers,
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
    pub destination: Vec32Registers,
    pub address: AddressOperand,
    pub source: Vec32Registers,
    pub cache_policy: Option<RegisterOperand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semantics {
    /// `.relaxed`
    Relaxed,
    /// `.acquire`
    Acquire,
    /// `.release`
    Release,
    /// `.acq_rel`
    AcqRel,
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
    /// `.shared{::cta, ::cluster}`
    Shared(SharedSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorStateSpace {
    /// `.global`
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSpace {
    /// `.shared::cta`
    Cta,
    /// `.shared::cluster`
    Cluster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheHint {
    /// `.L2::cache_hint`
    L2CacheHint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
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
pub enum ScalarOperation {
    /// `.and`
    And,
    /// `.or`
    Or,
    /// `.xor`
    Xor,
    /// `.exch`
    Exch,
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
pub enum CompareSwapVariant {
    /// `.op.type`
    Typed(DataType),
    /// `.cas.b16`
    B16,
    /// `.cas.b128`
    B128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoFtzType {
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
