use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vset4 {
    /// `vset4.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;`
    SimdMerge {
        /// `.atype`
        a_type: OperandType,
        /// `.btype`
        b_type: OperandType,
        /// `.cmp`
        compare_op: CompareOp,
        /// `d{.mask}`
        destination: Destination,
        /// `a{.asel}`
        a: SourceA,
        /// `b{.bsel}`
        b: SourceB,
        /// `c`
        c: RegisterOperand,
    },
    /// `vset4.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;`
    Accumulate {
        /// `.atype`
        a_type: OperandType,
        /// `.btype`
        b_type: OperandType,
        /// `.cmp`
        compare_op: CompareOp,
        /// `d{.mask}`
        destination: Destination,
        /// `a{.asel}`
        a: SourceA,
        /// `b{.bsel}`
        b: SourceB,
        /// `c`
        c: RegisterOperand,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    /// `d`
    pub register: RegisterOperand,
    /// `.mask` defaults to `.b3210` when `None`.
    pub mask: Option<Mask>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceA {
    /// `a`
    pub register: RegisterOperand,
    /// `.asel` defaults to `.b3210` when `None`.
    pub selector: Option<Selector>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceB {
    /// `b`
    pub register: RegisterOperand,
    /// `.bsel` defaults to `.b7654` when `None`.
    pub selector: Option<Selector>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    /// `.eq`
    Eq,
    /// `.ne`
    Ne,
    /// `.lt`
    Lt,
    /// `.le`
    Le,
    /// `.gt`
    Gt,
    /// `.ge`
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mask {
    /// `.b0`
    B0,
    /// `.b1`
    B1,
    /// `.b10`
    B10,
    /// `.b2`
    B2,
    /// `.b20`
    B20,
    /// `.b21`
    B21,
    /// `.b210`
    B210,
    /// `.b3`
    B3,
    /// `.b30`
    B30,
    /// `.b31`
    B31,
    /// `.b310`
    B310,
    /// `.b32`
    B32,
    /// `.b320`
    B320,
    /// `.b321`
    B321,
    /// `.b3210`
    B3210,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selector {
    /// `x`
    pub x: Lane,
    /// `y`
    pub y: Lane,
    /// `z`
    pub z: Lane,
    /// `w`
    pub w: Lane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    /// `0`
    B0,
    /// `1`
    B1,
    /// `2`
    B2,
    /// `3`
    B3,
    /// `4`
    B4,
    /// `5`
    B5,
    /// `6`
    B6,
    /// `7`
    B7,
}
