use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vmad {
    /// `vmad.dtype.atype.btype{.sat}{.scale} d, {-}a{.asel}, {-}b{.bsel}, {-}c;`
    Standard(Standard),
    /// `vmad.dtype.atype.btype.po{.sat}{.scale} d, a{.asel}, b{.bsel}, c;`
    PlusOne(PlusOne),
}

/// `vmad.dtype.atype.btype{.sat}{.scale} d, {-}a{.asel}, {-}b{.bsel}, {-}c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standard {
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.scale`
    pub scale: Option<Scale>,
    /// `d`
    pub destination: RegisterOperand,
    /// `{-}`
    pub a_negated: bool,
    /// `a`
    pub a: RegisterOperand,
    /// `{.asel}`
    pub asel: Option<ComponentSelect>,
    /// `{-}`
    pub b_negated: bool,
    /// `b`
    pub b: RegisterOperand,
    /// `{.bsel}`
    pub bsel: Option<ComponentSelect>,
    /// `{-}`
    pub c_negated: bool,
    /// `c`
    pub c: RegisterOperand,
}

/// `vmad.dtype.atype.btype.po{.sat}{.scale} d, a{.asel}, b{.bsel}, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlusOne {
    /// `.dtype`
    pub dtype: DataType,
    /// `.atype`
    pub atype: DataType,
    /// `.btype`
    pub btype: DataType,
    /// `.sat`
    pub saturate: bool,
    /// `.scale`
    pub scale: Option<Scale>,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `{.asel}`
    pub asel: Option<ComponentSelect>,
    /// `b`
    pub b: RegisterOperand,
    /// `{.bsel}`
    pub bsel: Option<ComponentSelect>,
    /// `c`
    pub c: RegisterOperand,
}

/// `.dtype = .atype = .btype = { .u32, .s32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}

/// `.scale = { .shr7, .shr15 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    /// `.shr7`
    Shr7,
    /// `.shr15`
    Shr15,
}

/// `.asel = .bsel = { .b0, .b1, .b2, .b3, .h0, .h1 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSelect {
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
