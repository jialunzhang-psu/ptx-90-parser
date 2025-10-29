use crate::r#type::common::RegisterOperand;

/// `cnot.type d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cnot {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.type = { .b16, .b32, .b64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
