use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vote {
    /// `vote.mode.pred d, {!}a;`
    Predicate(Predicate),
    /// `vote.ballot.b32 d, {!}a;`
    Ballot(Ballot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    /// `.mode`
    pub mode: Mode,
    /// `d`
    pub destination: PredicateRegister,
    /// `{!}a`
    pub source: PredicateOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ballot {
    /// `d`
    pub destination: RegisterOperand,
    /// `{!}a`
    pub source: PredicateOperand,
}

/// `.mode = { .all, .any, .uni };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.all`
    All,
    /// `.any`
    Any,
    /// `.uni`
    Uni,
}

/// `{!}a`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateOperand {
    /// `a`
    pub register: PredicateRegister,
    /// `!`
    pub negated: bool,
}
