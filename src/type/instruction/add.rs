use crate::r#type::common::*;

/// `add.type d, a, b;`
/// `add{.sat}.s32 d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub a: RegisterOperand,
    pub b: Operand,
}

/// `.type = { .u16, .u32, .u64, .s16, .s32, .s64, .u16x2, .s16x2 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s16`
    S16,
    /// `.s32`
    S32 {
        /// `.sat`
        saturate: bool,
    },
    /// `.s64`
    S64,
    /// `.u16x2`
    U16x2,
    /// `.s16x2`
    S16x2,
}
