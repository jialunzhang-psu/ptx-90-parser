use crate::r#type::common::RegisterOperand;

/// `addc{.cc}.type d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Addc {
    /// `.cc`
    pub condition_code: ConditionCode,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub augend: RegisterOperand,
    /// `b`
    pub addend: RegisterOperand,
}

/// `.cc`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCode {
    None,
    /// `.cc`
    Cc,
}

/// `.type = { .u32, .s32, .u64, .s64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
    /// `.u64`
    U64,
    /// `.s64`
    S64,
}
