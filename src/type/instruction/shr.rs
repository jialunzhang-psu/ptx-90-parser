use crate::r#type::common::*;

/// `shr.type d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shr {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: Operand,
}

/// `.type = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
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
