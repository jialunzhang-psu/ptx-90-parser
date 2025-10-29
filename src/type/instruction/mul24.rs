use crate::r#type::common::RegisterOperand;

/// `mul24.mode.type d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mul24 {
    pub mode: Mode,
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub a: RegisterOperand,
    pub b: RegisterOperand,
}

/// `.mode = { .hi, .lo };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.hi`
    Hi,
    /// `.lo`
    Lo,
}

/// `.type = { .u32, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}
