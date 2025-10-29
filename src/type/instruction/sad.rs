use crate::r#type::common::RegisterOperand;

/// `sad.type  d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sad {
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub a: RegisterOperand,
    pub b: RegisterOperand,
    pub c: RegisterOperand,
}

/// `.type = { .u16, .u32, .u64, .s16, .s32, .s64 };`
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
