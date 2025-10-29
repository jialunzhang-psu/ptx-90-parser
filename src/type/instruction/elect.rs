use crate::r#type::common::*;

/// `elect.sync d|p, membermask;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elect {
    /// `d`
    pub destination: Destination,
    /// `p`
    pub predicate: PredicateRegister,
    /// `membermask`
    pub member_mask: Operand,
}

/// `d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    Register(RegisterOperand),
    /// `_`
    Sink,
}
