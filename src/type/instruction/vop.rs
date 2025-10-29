//! Scalar Video Instructions
//! This is a common definition for vadd, vsub, vabsdiff, vmin, vmax
use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vop {
    /// `vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};`
    Scalar(Scalar),
    /// `vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;`
    ScalarWithSecondary(ScalarWithSecondary),
    /// `vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;`
    DataMerge(DataMerge),
}

/// `vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `d`
    pub d: RegisterOperand,
    /// `a{.asel}`
    pub a: Operand,
    /// `b{.bsel}`
    pub b: Operand,
}

/// `vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarWithSecondary {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.op2`
    pub op2: SecondaryOpcode,
    /// `d`
    pub d: RegisterOperand,
    /// `a{.asel}`
    pub a: Operand,
    /// `b{.bsel}`
    pub b: Operand,
    /// `c`
    pub c: RegisterOperand,
}

/// `vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMerge {
    /// `vop`
    pub opcode: Opcode,
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `d.dsel`
    pub d: MergeDestination,
    /// `a{.asel}`
    pub a: Operand,
    /// `b{.bsel}`
    pub b: Operand,
    /// `c`
    pub c: RegisterOperand,
}

/// `x{.xsel}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operand {
    /// `x`
    pub register: RegisterOperand,
    /// `.xsel`
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

/// `vop = { vadd, vsub, vabsdiff, vmin, vmax }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// `vadd`
    Vadd,
    /// `vsub`
    Vsub,
    /// `vabsdiff`
    Vabsdiff,
    /// `vmin`
    Vmin,
    /// `vmax`
    Vmax,
}

/// `.dtype = .atype = .btype = { .u32, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `.dsel = .asel = .bsel = { .b0, .b1, .b2, .b3, .h0, .h1 }`
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

/// `.op2 = { .add, .min, .max }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryOpcode {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
}
