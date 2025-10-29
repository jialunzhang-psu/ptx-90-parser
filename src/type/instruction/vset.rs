use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vset {
    /// `vset.atype.btype.cmp       d, a{.asel}, b{.bsel};`
    Scalar(Scalar),
    /// `vset.atype.btype.cmp.op2   d, a{.asel}, b{.bsel}, c;`
    ScalarWithSecondary(ScalarWithSecondary),
    /// `vset.atype.btype.cmp  d.dsel, a{.asel}, b{.bsel}, c;`
    DataMerge(DataMerge),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.cmp`
    pub cmp: Compare,
    /// `d`
    pub d: RegisterOperand,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarWithSecondary {
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.cmp`
    pub cmp: Compare,
    /// `.op2`
    pub op2: SecondaryOperation,
    /// `d`
    pub d: RegisterOperand,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
    /// `c`
    pub c: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMerge {
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.cmp`
    pub cmp: Compare,
    /// `d`
    pub d: RegisterOperand,
    /// `.dsel`
    pub dsel: Selection,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
    /// `c`
    pub c: RegisterOperand,
}

/// `a{.asel}`
/// `b{.bsel}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// `a`
    /// `b`
    pub register: RegisterOperand,
    /// `.asel`
    /// `.bsel`
    pub selection: Option<Selection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compare {
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
pub enum Selection {
    /// `.b0`
    B0,
    /// `.b1`
    B1,
    /// `.b2`
    B2,
    /// `.b3`
    B3,
    /// `.h0`
    H0,
    /// `.h1`
    H1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryOperation {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
}
