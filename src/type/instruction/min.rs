use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Min {
    /// `min.atype d, a, b;`
    AType {
        /// `.atype`
        data_type: AType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `min{.relu}.btype d, a, b;`
    BType {
        /// `.relu`
        relu: bool,
        /// `.btype`
        data_type: BType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
}

/// `.atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AType {
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.u16x2`
    U16x2,
    /// `.s16`
    S16,
    /// `.s64`
    S64,
}

/// `.btype = { .s16x2, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BType {
    /// `.s16x2`
    S16x2,
    /// `.s32`
    S32,
}
