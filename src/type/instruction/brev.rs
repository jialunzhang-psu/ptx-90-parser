use crate::r#type::common::RegisterOperand;

/// `brev.type  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brev {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.type = { .b32, .b64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
