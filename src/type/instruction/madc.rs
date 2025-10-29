use crate::r#type::common::RegisterOperand;

/// `madc{.hi,.lo}{.cc}.type d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Madc {
    /// `{.hi,.lo}`
    pub result_part: Option<ResultPart>,
    /// `{.cc}`
    pub condition_code: bool,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub multiplicand: RegisterOperand,
    /// `b`
    pub multiplier: RegisterOperand,
    /// `c`
    pub addend: RegisterOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultPart {
    /// `.hi`
    Hi,
    /// `.lo`
    Lo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
    /// `.u64`
    U64,
    /// `.s64`
    S64,
}
