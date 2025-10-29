use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Match {
    /// `match.any.sync.type d, a, membermask;`
    Any(Any),
    /// `match.all.sync.type d[|p], a, membermask;`
    All(All),
}

/// `match.any.sync.type d, a, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Any {
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
    /// `membermask`
    pub member_mask: Operand,
}

/// `match.all.sync.type d[|p], a, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct All {
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub source: RegisterOperand,
    /// `membermask`
    pub member_mask: Operand,
}

/// `.type = { .b32, .b64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}
