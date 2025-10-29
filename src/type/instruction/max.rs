use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Max {
    /// `max.atype d, a, b;`
    AType {
        data_type: AType,
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
    },
    /// `max{.relu}.btype d, a, b;`
    BType {
        relu: Relu,
        data_type: BType,
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
    },
}

/// `{.relu}` in `max{.relu}.btype d, a, b;`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relu {
    Default,
    /// `.relu`
    Relu,
}

/// `.atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 };`
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

/// `.btype = { .s16x2, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BType {
    /// `.s16x2`
    S16x2,
    /// `.s32`
    S32,
}
