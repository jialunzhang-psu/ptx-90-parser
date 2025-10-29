//! vop4 instructions, the SIMD video instructions operate on quadruples of 8-bit values
//! This is a meta-instruction that includes vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4
use crate::r#type::common::RegisterOperand;

/// `vop4.dtype.atype.btype{.sat} d{.mask}, a{.asel}, b{.bsel}, c;`
/// `vop4.dtype.atype.btype.add d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vop4 {
    /// `vop4.dtype.atype.btype{.sat} d{.mask}, a{.asel}, b{.bsel}, c;`
    Merge(Merge),
    /// `vop4.dtype.atype.btype.add d{.mask}, a{.asel}, b{.bsel}, c;`
    Accumulate(Accumulate),
}

/// `vop4.dtype.atype.btype{.sat} d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Merge {
    /// `vop4 = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 }`
    pub operation: Operation,
    /// `.dtype`
    pub data_type: DataType,
    /// `.atype`
    pub a_type: DataType,
    /// `.btype`
    pub b_type: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `d{.mask}`
    pub destination: Destination,
    /// `a{.asel}`
    pub a: ASource,
    /// `b{.bsel}`
    pub b: BSource,
    /// `c`
    pub c: RegisterOperand,
}

/// `vop4.dtype.atype.btype.add d{.mask}, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accumulate {
    /// `vop4 = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 }`
    pub operation: Operation,
    /// `.dtype`
    pub data_type: DataType,
    /// `.atype`
    pub a_type: DataType,
    /// `.btype`
    pub b_type: DataType,
    /// `d{.mask}`
    pub destination: Destination,
    /// `a{.asel}`
    pub a: ASource,
    /// `b{.bsel}`
    pub b: BSource,
    /// `c`
    pub c: RegisterOperand,
}

/// `d{.mask}` with `.mask` defaulting to `.b3210`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    /// `d`
    pub register: RegisterOperand,
    /// `{.mask}`
    pub mask: Option<Mask>,
}

/// `a{.asel}` with `.asel` defaulting to `.b3210`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASource {
    /// `a`
    pub register: RegisterOperand,
    /// `{.asel}`
    pub selector: Option<Selector>,
}

/// `b{.bsel}` with `.bsel` defaulting to `.b7654`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BSource {
    /// `b`
    pub register: RegisterOperand,
    /// `{.bsel}`
    pub selector: Option<Selector>,
}

/// `.bxyzw` where each lane index is from `{ 0, ..., 7 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selector {
    /// `xyzw`
    pub lanes: [Lane; 4],
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

/// `.dtype = .atype = .btype = { .u32, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `vop4 = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// `vadd4`
    Vadd4,
    /// `vsub4`
    Vsub4,
    /// `vavrg4`
    Vavrg4,
    /// `vabsdiff4`
    Vabsdiff4,
    /// `vmin4`
    Vmin4,
    /// `vmax4`
    Vmax4,
}
