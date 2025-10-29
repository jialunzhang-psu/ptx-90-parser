use crate::r#type::common::*;

/// `shfl.mode.b32  d[|p], a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shfl {
    /// `.mode`
    pub mode: Mode,
    /// `.b32`
    pub data_type: DataType,
    /// `d[|p]`
    pub destination: Destination,
    /// `a`
    pub source: RegisterOperand,
    /// `b`
    pub lane: Operand,
    /// `c`
    pub clamp: Operand,
}

/// `.mode = { .up, .down, .bfly, .idx };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.up`
    Up,
    /// `.down`
    Down,
    /// `.bfly`
    Bfly,
    /// `.idx`
    Idx,
}

/// `.b32`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
}

/// `d[|p]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    /// `d`
    pub register: RegisterOperand,
    /// `|p`
    pub predicate: Option<PredicateRegister>,
}
