//! vop2 instructions, the SIMD video instructions operate on pairs of 16-bit values
//! This is a meta-instruction that includes vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2
use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vop2 {
    /// `vop2.dtype.atype.btype{.sat} d{.mask}, a{.asel}, b{.bsel}, c;`
    Merge(Merge),
    /// `vop2.dtype.atype.btype.add d{.mask}, a{.asel}, b{.bsel}, c;`
    Accumulate(Accumulate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Merge {
    /// `vop2 = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 }`
    pub operation: Operation,
    /// `.dtype = { .u32, .s32 }`
    pub dtype: DataType,
    /// `.atype = { .u32, .s32 }`
    pub atype: DataType,
    /// `.btype = { .u32, .s32 }`
    pub btype: DataType,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accumulate {
    /// `vop2 = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 }`
    pub operation: Operation,
    /// `.dtype = { .u32, .s32 }`
    pub dtype: DataType,
    /// `.atype = { .u32, .s32 }`
    pub atype: DataType,
    /// `.btype = { .u32, .s32 }`
    pub btype: DataType,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// `vadd2`
    Vadd2,
    /// `vsub2`
    Vsub2,
    /// `vavrg2`
    Vavrg2,
    /// `vabsdiff2`
    Vabsdiff2,
    /// `vmin2`
    Vmin2,
    /// `vmax2`
    Vmax2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mask {
    /// `.h0`
    H0,
    /// `.h1`
    H1,
    /// `.h10`
    H10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selector {
    /// `.asel = .bsel = { .hxy, where x,y are from { 0, 1, 2, 3 } }`
    ///
    /// `xy`
    pub halves: [Half; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    /// `x`, `y` can be `0`
    H0,
    /// `x`, `y` can be `1`
    H1,
    /// `x`, `y` can be `2`
    H2,
    /// `x`, `y` can be `3`
    H3,
}
