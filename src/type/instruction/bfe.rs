use crate::r#type::common::RegisterOperand;

/// `bfe.type d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bfe {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
    /// `b`
    pub bit_position: RegisterOperand,
    /// `c`
    pub field_length: RegisterOperand,
}

/// `.type = { .u32, .u64, .s32, .s64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
}
