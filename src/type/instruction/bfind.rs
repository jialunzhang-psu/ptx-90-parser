use crate::r#type::common::RegisterOperand;

/// `bfind.type d, a;`
/// `bfind.shiftamt.type d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bfind {
    /// `bfind.type d, a;`
    Plain {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
    /// `bfind.shiftamt.type d, a;`
    ShiftAmount {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
}

/// `.type = { .u32, .u64, .s32, .s64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
}
