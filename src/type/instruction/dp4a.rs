use crate::r#type::common::RegisterOperand;

/// `dp4a.atype.btype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dp4a {
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `c`
    pub c: RegisterOperand,
}

/// `.atype = .btype = { .u32, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}
