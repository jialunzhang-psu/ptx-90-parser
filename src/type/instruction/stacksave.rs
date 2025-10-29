use crate::r#type::common::RegisterOperand;

/// `stacksave.type d;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stacksave {
    pub data_type: DataType,
    pub destination: RegisterOperand,
}

/// `.type = { .u32, .u64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    U32,
    U64,
}
