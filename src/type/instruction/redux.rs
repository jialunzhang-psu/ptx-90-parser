use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Redux {
    /// `redux.sync.op.type dst, src, membermask;`
    Integer(Integer),
    /// `redux.sync.op.b32 dst, src, membermask;`
    Bitwise(Bitwise),
    /// `redux.sync.op{.abs.}{.NaN}.f32 dst, src, membermask;`
    Float(Float),
}

/// `redux.sync.op.type dst, src, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    /// `.op`
    pub operator: IntegerOperator,
    /// `.type`
    pub data_type: DataType,
    /// `dst`
    pub destination: RegisterOperand,
    /// `src`
    pub source: RegisterOperand,
    /// `membermask`
    pub member_mask: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerOperator {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
}

/// `.type = { .u32, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `redux.sync.op.b32 dst, src, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitwise {
    /// `.op`
    pub operator: BitwiseOperator,
    /// `dst`
    pub destination: RegisterOperand,
    /// `src`
    pub source: RegisterOperand,
    /// `membermask`
    pub member_mask: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseOperator {
    /// `.and`
    And,
    /// `.or`
    Or,
    /// `.xor`
    Xor,
}

/// `redux.sync.op{.abs.}{.NaN}.f32 dst, src, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float {
    /// `.op`
    pub operator: FloatOperator,
    /// `.abs`
    pub absolute: bool,
    /// `.NaN`
    pub propagate_nan: bool,
    /// `dst`
    pub destination: RegisterOperand,
    /// `src`
    pub source: RegisterOperand,
    /// `membermask`
    pub member_mask: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatOperator {
    /// `.min`
    Min,
    /// `.max`
    Max,
}
