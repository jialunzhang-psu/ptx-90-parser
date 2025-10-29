use crate::r#type::common::*;

/// `alloca.type  ptr, size{, immAlign};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alloca {
    /// `alloca.type  ptr, size;`
    Default {
        /// `.type`
        data_type: DataType,
        /// `ptr`
        pointer: RegisterOperand,
        /// `size`
        size: RegisterOperand,
    },
    /// `alloca.type  ptr, size, immAlign;`
    Aligned {
        /// `.type`
        data_type: DataType,
        /// `ptr`
        pointer: RegisterOperand,
        /// `size`
        size: RegisterOperand,
        /// `immAlign`
        alignment: Immediate,
    },
}

/// `.type = { .u32, .u64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
}
