use crate::r#type::common::RegisterOperand;

/// `neg.type  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neg {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.type = { .s16, .s32, .s64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
}
