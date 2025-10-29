use crate::r#type::common::{PredicateRegister, RegisterOperand};

/// `set.CmpOp{.ftz}.dtype.stype d, a, b;`
/// `set.CmpOp.BoolOp{.ftz}.dtype.stype d, a, b, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Set {
    /// `set.CmpOp{.ftz}.dtype.stype d, a, b;`
    Compare(Compare),
    /// `set.CmpOp.BoolOp{.ftz}.dtype.stype d, a, b, {!}c;`
    CompareBool(CompareBool),
}

/// `set.CmpOp{.ftz}.dtype.stype d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compare {
    /// `CmpOp`
    pub compare_op: CompareOp,
    /// `{.ftz}`
    pub flush_to_zero: bool,
    /// `.dtype`
    pub destination_type: DestinationType,
    /// `.stype`
    pub source_type: SourceType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
}

/// `set.CmpOp.BoolOp{.ftz}.dtype.stype d, a, b, {!}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareBool {
    /// `CmpOp`
    pub compare_op: CompareOp,
    /// `BoolOp`
    pub bool_op: BoolOp,
    /// `{.ftz}`
    pub flush_to_zero: bool,
    /// `.dtype`
    pub destination_type: DestinationType,
    /// `.stype`
    pub source_type: SourceType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `{!}c`
    pub predicate: Predicate,
}

/// `{!}c`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    /// `c`
    pub register: PredicateRegister,
    /// `!`
    pub negated: bool,
}

/// `.CmpOp = { eq, ne, lt, le, gt, ge, lo, ls, hi, hs, equ, neu, ltu, leu, gtu, geu, num, nan }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    /// `eq`
    Eq,
    /// `ne`
    Ne,
    /// `lt`
    Lt,
    /// `le`
    Le,
    /// `gt`
    Gt,
    /// `ge`
    Ge,
    /// `lo`
    Lo,
    /// `ls`
    Ls,
    /// `hi`
    Hi,
    /// `hs`
    Hs,
    /// `equ`
    Equ,
    /// `neu`
    Neu,
    /// `ltu`
    Ltu,
    /// `leu`
    Leu,
    /// `gtu`
    Gtu,
    /// `geu`
    Geu,
    /// `num`
    Num,
    /// `nan`
    Nan,
}

/// `.BoolOp = { and, or, xor }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    /// `and`
    And,
    /// `or`
    Or,
    /// `xor`
    Xor,
}

/// `.dtype = { .u32, .s32, .f32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
    /// `.f32`
    F32,
}

/// `.stype = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
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
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}
