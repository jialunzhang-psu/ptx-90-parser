use crate::r#type::common::RegisterOperand;

/// `bfi.type f, a, b, c, d;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bfi {
    /// `.type`
    pub data_type: DataType,
    /// `f`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
    /// `b`
    pub base: RegisterOperand,
    /// `c`
    pub position: RegisterOperand,
    /// `d`
    pub length: RegisterOperand,
}

/// `.type = { .b32, .b64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
