use crate::r#type::common::RegisterOperand;

/// `rem.type  d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rem {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub lhs: RegisterOperand,
    /// `b`
    pub rhs: RegisterOperand,
}

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
    S32,
    /// `.s64`
    S64,
}
