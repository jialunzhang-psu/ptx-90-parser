use crate::r#type::common::RegisterOperand;

/// `or.type d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Or {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
}

/// `.type = { .pred, .b16, .b32, .b64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.pred`
    Pred,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
