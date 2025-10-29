use crate::r#type::common::{PredicateRegister, RegisterOperand};

/// `setp.CmpOp{.ftz}.type p[|q], a, b;`
/// `setp.CmpOp.BoolOp{.ftz}.type p[|q], a, b, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Setp {
    /// `setp.CmpOp{.ftz}.type p[|q], a, b;`
    Compare(Compare),
    /// `setp.CmpOp.BoolOp{.ftz}.type p[|q], a, b, {!}c;`
    CompareBool(CompareBool),
}

/// `setp.CmpOp{.ftz}.type p[|q], a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compare {
    pub compare_op: CompareOp,
    pub flush_to_zero: bool,
    pub data_type: DataType,
    pub destination: Destination,
    pub a: RegisterOperand,
    pub b: RegisterOperand,
}

/// `setp.CmpOp.BoolOp{.ftz}.type p[|q], a, b, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareBool {
    pub compare_op: CompareOp,
    pub bool_op: BoolOp,
    pub flush_to_zero: bool,
    pub data_type: DataType,
    pub destination: Destination,
    pub a: RegisterOperand,
    pub b: RegisterOperand,
    pub predicate: Predicate,
}

/// `p[|q]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    pub predicate: PredicateTarget,
    pub complement: Option<PredicateTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateTarget {
    Register(PredicateRegister),
    Sink,
}

/// `{!}c`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub register: PredicateRegister,
    pub negated: bool,
}

/// `.CmpOp = { eq, ne, lt, le, gt, ge, lo, ls, hi, hs, equ, neu, ltu, leu, gtu, geu, num, nan }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Lo,
    Ls,
    Hi,
    Hs,
    Equ,
    Neu,
    Ltu,
    Leu,
    Gtu,
    Geu,
    Num,
    Nan,
}

/// `.BoolOp = { and, or, xor }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    And,
    Or,
    Xor,
}

/// `.type = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    B16,
    B32,
    B64,
    U16,
    U32,
    U64,
    S16,
    S32,
    S64,
    F32,
    F64,
}
