use crate::r#type::common::RegisterOperand;

/// `szext.mode.type d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Szext {
    /// `.mode`
    pub mode: Mode,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
}

/// `.mode = { .clamp, .wrap };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.clamp`
    Clamp,
    /// `.wrap`
    Wrap,
}

/// `.type = { .u32, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}
