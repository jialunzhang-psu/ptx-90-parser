use crate::r#type::common::RegisterOperand;

/// `mul.mode.type  d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mul {
    pub mode: Mode,
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub lhs: RegisterOperand,
    pub rhs: RegisterOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.hi`
    Hi,
    /// `.lo`
    Lo,
    /// `.wide`
    Wide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
}
