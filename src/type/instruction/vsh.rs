//! Scalar Video Instructions: Integer byte/half-word/word left/right shift.
//! This is a common definition for vshl, vshr
use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vsh {
    /// `vop.dtype.atype.u32{.sat}.mode       d, a{.asel}, b{.bsel};`
    Scalar(Scalar),
    /// `vop.dtype.atype.u32{.sat}.mode.op2   d, a{.asel}, b{.bsel}, c;`
    ScalarWithSecondary(ScalarWithSecondary),
    /// `vop.dtype.atype.u32{.sat}.mode  d.dsel, a{.asel}, b{.bsel}, c;`
    DataMerge(DataMerge),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.mode`
    pub mode: Mode,
    /// `d`
    pub destination: RegisterOperand,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarWithSecondary {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.mode`
    pub mode: Mode,
    /// `.op2`
    pub secondary: SecondaryOp,
    /// `d`
    pub destination: RegisterOperand,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
    /// `c`
    pub c: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMerge {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.mode`
    pub mode: Mode,
    /// `d.dsel`
    pub destination: MergeDestination,
    /// `a{.asel}`
    pub a: Source,
    /// `b{.bsel}`
    pub b: Source,
    /// `c`
    pub c: RegisterOperand,
}

/// `a{.asel}`/`b{.bsel}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// `a`/`b`
    pub register: RegisterOperand,
    /// `{.asel}`/`{.bsel}`
    pub selection: Option<Selection>,
}

/// `d.dsel`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeDestination {
    /// `d`
    pub register: RegisterOperand,
    /// `.dsel`
    pub selection: Selection,
}

/// `vop = { vshl, vshr };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// `vshl`
    Vshl,
    /// `vshr`
    Vshr,
}

/// `.dtype = .atype = { .u32, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `.mode  = { .clamp, .wrap };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.clamp`
    Clamp,
    /// `.wrap`
    Wrap,
}

/// `.dsel = .asel = .bsel = { .b0, .b1, .b2, .b3, .h0, .h1 };`
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

/// `.op2   = { .add, .min, .max };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryOp {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
}
