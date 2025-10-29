use crate::r#type::common::{AddressOperand, RegisterOperand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// `multimem.ld_reduce{.ldsem}{.scope}{.ss}.op.type      d, [a];`
    /// `multimem.ld_reduce{.ldsem}{.scope}{.ss}.op{.acc_prec}{.vec}.type d, [a];`
    /// `multimem.ld_reduce.weak{.ss}.op.type                 d, [a];`
    /// `multimem.ld_reduce.weak{.ss}.op{.acc_prec}{.vec}.type d, [a];`
    LdReduce(LdReduce),
    /// `multimem.st{.stsem}{.scope}{.ss}.type                [a], b;`
    /// `multimem.st{.stsem}{.scope}{.ss}{.vec}.type          [a], b;`
    /// `multimem.st.weak{.ss}.type                           [a], b;`
    /// `multimem.st.weak{.ss}{.vec}.type                     [a], b;`
    Store(Store),
    /// `multimem.red{.redsem}{.scope}{.ss}.op.type           [a], b;`
    /// `multimem.red{.redsem}{.scope}{.ss}.redop{.vec}.redtype [a], b;`
    Red(Red),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LdReduce {
    /// `multimem.ld_reduce{.ldsem}{.scope}{.ss}.op.type      d, [a];`
    Int(LdReduceInt),
    /// `multimem.ld_reduce{.ldsem}{.scope}{.ss}.op{.acc_prec}{.vec}.type d, [a];`
    Float(LdReduceFloat),
    /// `multimem.ld_reduce.weak{.ss}.op.type                 d, [a];`
    WeakInt(LdReduceWeakInt),
    /// `multimem.ld_reduce.weak{.ss}.op{.acc_prec}{.vec}.type d, [a];`
    WeakFloat(LdReduceWeakFloat),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdReduceInt {
    pub semantics: Option<LoadSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: IntegerOp,
    pub data_type: IntegerType,
    pub destination: RegisterOperand,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdReduceFloat {
    pub semantics: Option<LoadSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: FloatOp,
    pub accumulator_precision: Option<AccumulatorPrecision>,
    pub vector: Option<VectorWidth>,
    pub data_type: FloatType,
    pub destination: VectorDestination,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdReduceWeakInt {
    pub state_space: Option<StateSpace>,
    pub operation: IntegerOp,
    pub data_type: IntegerType,
    pub destination: RegisterOperand,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdReduceWeakFloat {
    pub state_space: Option<StateSpace>,
    pub operation: FloatOp,
    pub accumulator_precision: Option<AccumulatorPrecision>,
    pub vector: Option<VectorWidth>,
    pub data_type: FloatType,
    pub destination: VectorDestination,
    pub address: AddressOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Store {
    /// `multimem.st{.stsem}{.scope}{.ss}.type                [a], b;`
    Int(StoreInt),
    /// `multimem.st{.stsem}{.scope}{.ss}{.vec}.type          [a], b;`
    Float(StoreFloat),
    /// `multimem.st.weak{.ss}.type                           [a], b;`
    WeakInt(StoreWeakInt),
    /// `multimem.st.weak{.ss}{.vec}.type                     [a], b;`
    WeakFloat(StoreWeakFloat),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreInt {
    pub semantics: Option<StoreSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub data_type: IntegerType,
    pub address: AddressOperand,
    pub value: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreFloat {
    pub semantics: Option<StoreSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub vector: Option<VectorWidth>,
    pub data_type: FloatType,
    pub address: AddressOperand,
    pub value: VectorValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreWeakInt {
    pub state_space: Option<StateSpace>,
    pub data_type: IntegerType,
    pub address: AddressOperand,
    pub value: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreWeakFloat {
    pub state_space: Option<StateSpace>,
    pub vector: Option<VectorWidth>,
    pub data_type: FloatType,
    pub address: AddressOperand,
    pub value: VectorValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Red {
    /// `multimem.red{.redsem}{.scope}{.ss}.op.type           [a], b;`
    Int(RedInt),
    /// `multimem.red{.redsem}{.scope}{.ss}.redop{.vec}.redtype [a], b;`
    Float(RedFloat),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedInt {
    pub semantics: Option<ReductionSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: IntegerOp,
    pub data_type: IntegerType,
    pub address: AddressOperand,
    pub value: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedFloat {
    pub semantics: Option<ReductionSemantics>,
    pub scope: Option<Scope>,
    pub state_space: Option<StateSpace>,
    pub operation: FloatRedOp,
    pub vector: Option<VectorWidth>,
    pub data_type: FloatReductionType,
    pub address: AddressOperand,
    pub value: VectorValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSemantics {
    Relaxed,
    Acquire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSemantics {
    Relaxed,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionSemantics {
    Relaxed,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Cta,
    Cluster,
    Gpu,
    Sys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerOp {
    Min,
    Max,
    Add,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatOp {
    Min,
    Max,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatRedOp {
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccumulatorPrecision {
    AccF32,
    AccF16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorWidth {
    V2,
    V4,
    V8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerType {
    B32,
    B64,
    U32,
    U64,
    S32,
    S64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatType {
    F16,
    F16x2,
    Bf16,
    Bf16x2,
    F32,
    F64,
    E5m2,
    E5m2x2,
    E5m2x4,
    E4m3,
    E4m3x2,
    E4m3x4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatReductionType {
    F16,
    F16x2,
    Bf16,
    Bf16x2,
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorDestination {
    Scalar(RegisterOperand),
    Vector2([RegisterOperand; 2]),
    Vector4([RegisterOperand; 4]),
    Vector8([RegisterOperand; 8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorValue {
    Scalar(RegisterOperand),
    Vector2([RegisterOperand; 2]),
    Vector4([RegisterOperand; 4]),
    Vector8([RegisterOperand; 8]),
}
