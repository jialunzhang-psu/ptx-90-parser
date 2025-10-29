use crate::r#type::common::RegisterOperand;

/// `stackrestore.type a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stackrestore {
    pub data_type: DataType,
    pub register: RegisterOperand,
}

/// `.type = { .u32, .u64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
}
