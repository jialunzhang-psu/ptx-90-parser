use crate::r#type::common::RegisterOperand;

/// `fns.b32 d, mask, base, offset;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fns {
    /// `d` (.b32)
    pub destination: RegisterOperand,
    /// `mask`
    pub mask: Mask,
    /// `base`
    pub base: Base,
    /// `offset` (.s32)
    pub offset: RegisterOperand,
}

/// `.mask = { .b32, .u32, .s32 }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mask {
    /// `.b32`
    B32(RegisterOperand),
    /// `.u32`
    U32(RegisterOperand),
    /// `.s32`
    S32(RegisterOperand),
}

/// `.base = { .b32, .u32, .s32 }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Base {
    /// `.b32`
    B32(RegisterOperand),
    /// `.u32`
    U32(RegisterOperand),
    /// `.s32`
    S32(RegisterOperand),
}
