use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rcp {
    /// `rcp.approx{.ftz}.f32  d, a;`
    ApproxF32(ApproxF32),
    /// `rcp.rnd{.ftz}.f32     d, a;`
    RndF32(RndF32),
    /// `rcp.rnd.f64           d, a;`
    RndF64(RndF64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproxF32 {
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RndF32 {
    /// `.rnd`
    pub rounding: Rounding,
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RndF64 {
    /// `.rnd`
    pub rounding: Rounding,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.rnd = { .rn, .rz, .rm, .rp }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rounding {
    /// `.rn`
    Rn,
    /// `.rz`
    Rz,
    /// `.rm`
    Rm,
    /// `.rp`
    Rp,
}
