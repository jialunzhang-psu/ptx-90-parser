use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rsqrt {
    /// `rsqrt.approx{.ftz}.f32  d, a;`
    ApproxF32(ApproxF32),
    /// `rsqrt.approx.f64        d, a;`
    ApproxF64(ApproxF64),
}

/// `rsqrt.approx{.ftz}.f32  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproxF32 {
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `rsqrt.approx.f64        d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApproxF64 {
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}
