use crate::r#type::common::{Immediate, PredicateRegister, RegisterOperand};

/// `lop3.b32 d, a, b, c, immLut;`
/// `lop3.BoolOp.b32 d|p, a, b, c, immLut, q;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lop3 {
    /// `lop3.b32 d, a, b, c, immLut;`
    Plain(Plain),
    /// `lop3.BoolOp.b32 d|p, a, b, c, immLut, q;`
    Boolean(Boolean),
}

/// `lop3.b32 d, a, b, c, immLut;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plain {
    /// `.b32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `c`
    pub c: RegisterOperand,
    /// `immLut`
    pub lut: Immediate,
}

/// `lop3.BoolOp.b32 d|p, a, b, c, immLut, q;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    /// `.BoolOp`
    pub operator: BoolOp,
    /// `.b32`
    pub data_type: DataType,
    /// `d`
    pub destination: Destination,
    /// `p`
    pub predicate: PredicateRegister,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `c`
    pub c: RegisterOperand,
    /// `immLut`
    pub lut: Immediate,
    /// `q`
    pub predicate_input: PredicateRegister,
}

/// `d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    /// `d`
    Register(RegisterOperand),
    /// `_`
    Sink,
}

/// `.BoolOp = { .or, .and }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    /// `.or`
    Or,
    /// `.and`
    And,
}

/// `.b32`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
}
