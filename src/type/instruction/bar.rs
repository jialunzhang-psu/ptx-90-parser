use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bar {
    /// `barrier{.cta}.sync{.aligned} a{, b};`
    BarrierSync(BarrierSync),
    /// `barrier{.cta}.arrive{.aligned} a, b;`
    BarrierArrive(BarrierArrive),
    /// `barrier{.cta}.red.popc{.aligned}.u32 d, a{, b}, {!}c;`
    BarrierReductionPopc(BarrierReductionPopc),
    /// `barrier{.cta}.red.op{.aligned}.pred p, a{, b}, {!}c;`
    BarrierReductionLogical(BarrierReductionLogical),
    /// `bar{.cta}.sync a{, b};`
    BarSync(BarSync),
    /// `bar{.cta}.arrive a, b;`
    BarArrive(BarArrive),
    /// `bar{.cta}.red.popc.u32 d, a{, b}, {!}c;`
    BarReductionPopc(BarReductionPopc),
    /// `bar{.cta}.red.op.pred p, a{, b}, {!}c;`
    BarReductionLogical(BarReductionLogical),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierSync {
    pub scope: Scope,
    pub aligned: bool,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierArrive {
    pub scope: Scope,
    pub aligned: bool,
    pub barrier: Operand,
    pub expected_count: Operand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierReductionPopc {
    pub scope: Scope,
    pub aligned: bool,
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
    pub predicate: PredicateInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarrierReductionLogical {
    pub scope: Scope,
    pub aligned: bool,
    pub destination: PredicateRegister,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
    pub predicate: PredicateInput,
    pub operation: LogicalOperation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarSync {
    pub scope: Scope,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarArrive {
    pub scope: Scope,
    pub barrier: Operand,
    pub expected_count: Operand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarReductionPopc {
    pub scope: Scope,
    pub data_type: DataType,
    pub destination: RegisterOperand,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
    pub predicate: PredicateInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarReductionLogical {
    pub scope: Scope,
    pub destination: PredicateRegister,
    pub barrier: Operand,
    pub expected_count: Option<Operand>,
    pub predicate: PredicateInput,
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
