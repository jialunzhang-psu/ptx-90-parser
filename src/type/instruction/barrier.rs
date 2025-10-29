use crate::r#type::common::{Operand, PredicateRegister, RegisterOperand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Barrier {
    /// `barrier{.cta}.sync{.aligned} a{, b};`
    Sync(BarrierSync),
    /// `barrier{.cta}.arrive{.aligned} a, b;`
    Arrive(BarrierArrive),
    /// `barrier{.cta}.red.popc{.aligned}.u32 d, a{, b}, {!}c;`
    ReductionPopc(BarrierReductionPopc),
    /// `barrier{.cta}.red.op{.aligned}.pred p, a{, b}, {!}c;`
    ReductionLogical(BarrierReductionLogical),
    /// `bar{.cta}.sync a{, b};`
    BarSync(BarSync),
    /// `bar{.cta}.arrive a, b;`
    BarArrive(BarArrive),
    /// `bar{.cta}.red.popc.u32 d, a{, b}, {!}c;`
    BarReductionPopc(BarReductionPopc),
    /// `bar{.cta}.red.op.pred p, a{, b}, {!}c;`
    BarReductionLogical(BarReductionLogical),
}

/// `barrier{.cta}.sync{.aligned} a{, b};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierSync {
    /// `.cta`
    pub scope: Scope,
    /// `.aligned`
    pub aligned: bool,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
}

/// `barrier{.cta}.arrive{.aligned} a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierArrive {
    /// `.cta`
    pub scope: Scope,
    /// `.aligned`
    pub aligned: bool,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Operand,
}

/// `barrier{.cta}.red.popc{.aligned}.u32 d, a{, b}, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierReductionPopc {
    /// `.cta`
    pub scope: Scope,
    /// `.aligned`
    pub aligned: bool,
    /// `.u32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
    /// `{!}c`
    pub predicate: PredicateInput,
}

/// `barrier{.cta}.red.op{.aligned}.pred p, a{, b}, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierReductionLogical {
    /// `.cta`
    pub scope: Scope,
    /// `.aligned`
    pub aligned: bool,
    /// `.pred`
    pub destination: PredicateRegister,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
    /// `{!}c`
    pub predicate: PredicateInput,
    /// `.op`
    pub operation: LogicalOperation,
}

/// `bar{.cta}.sync a{, b};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarSync {
    /// `.cta`
    pub scope: Scope,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
}

/// `bar{.cta}.arrive a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarArrive {
    /// `.cta`
    pub scope: Scope,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Operand,
}

/// `bar{.cta}.red.popc.u32 d, a{, b}, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarReductionPopc {
    /// `.cta`
    pub scope: Scope,
    /// `.u32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
    /// `{!}c`
    pub predicate: PredicateInput,
}

/// `bar{.cta}.red.op.pred p, a{, b}, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarReductionLogical {
    /// `.cta`
    pub scope: Scope,
    /// `.pred`
    pub destination: PredicateRegister,
    /// `a`
    pub barrier: Operand,
    /// `b`
    pub expected_count: Option<Operand>,
    /// `{!}c`
    pub predicate: PredicateInput,
    /// `.op`
    pub operation: LogicalOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// (absent)
    None,
    /// `.cta`
    Cta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
}

/// `.op = { .and, .or };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperation {
    /// `.and`
    And,
    /// `.or`
    Or,
}

/// `{!}c`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateInput {
    pub negated: bool,
    pub predicate: PredicateRegister,
}
