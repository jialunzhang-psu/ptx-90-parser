use crate::r#type::common::RegisterOperand;

/// `div.type  d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Div {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
}

/// `.type = { .u16, .u32, .u64, .s16, .s32, .s64 }`
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
