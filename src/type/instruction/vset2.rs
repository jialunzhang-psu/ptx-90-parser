use crate::r#type::common::RegisterOperand;

/// `vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;`
/// `vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vset2 {
    /// `vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;`
    SimdMerge(SimdMerge),
    /// `vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;`
    Accumulate(Accumulate),
}

/// `vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimdMerge {
    /// `.atype`
    pub a_type: DataType,
    /// `.btype`
    pub b_type: DataType,
    /// `.cmp`
    pub comparison: CompareOp,
    /// `d{.mask}`
    pub destination: Destination,
    /// `a{.asel}`
    pub a: ASource,
    /// `b{.bsel}`
    pub b: BSource,
    /// `c`
    pub c: RegisterOperand,
}

/// `vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accumulate {
    /// `.atype`
    pub a_type: DataType,
    /// `.btype`
    pub b_type: DataType,
    /// `.cmp`
    pub comparison: CompareOp,
    /// `d{.mask}`
    pub destination: Destination,
    /// `a{.asel}`
    pub a: ASource,
    /// `b{.bsel}`
    pub b: BSource,
    /// `c`
    pub c: RegisterOperand,
}

/// `d{.mask}` default `.h10`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    /// `d`
    pub register: RegisterOperand,
    /// `{.mask}`
    pub mask: Option<Mask>,
}

/// `a{.asel}` default `.h10`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASource {
    /// `a`
    pub register: RegisterOperand,
    /// `{.asel}`
    pub selector: Option<Selector>,
}

/// `b{.bsel}` default `.h32`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BSource {
    /// `b`
    pub register: RegisterOperand,
    /// `{.bsel}`
    pub selector: Option<Selector>,
}

/// `.atype = .btype = { .u32, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `.cmp = { .eq, .ne, .lt, .le, .gt, .ge }`
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

/// `.mask = { .h0, .h1, .h10 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mask {
    /// `.h0`
    H0,
    /// `.h1`
    H1,
    /// `.h10`
    H10,
}

/// `.asel = .bsel = { .hxy, where x,y are from { 0, 1, 2, 3 } }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selector {
    /// `xy`
    pub halves: [Half; 2],
}

/// `x`, `y` from `{ 0, 1, 2, 3 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    /// `0`
    H0,
    /// `1`
    H1,
    /// `2`
    H2,
    /// `3`
    H3,
}
