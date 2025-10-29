use crate::r#type::common::RegisterOperand;

/// `clz.type d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clz {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
