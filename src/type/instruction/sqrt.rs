use crate::r#type::common::RegisterOperand;

/// `sqrt.approx{.ftz}.f32 d, a;`
/// `sqrt.rnd{.ftz}.f32 d, a;`
/// `sqrt.rnd.f64 d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sqrt {
    /// `sqrt.approx{.ftz}.f32 d, a;`
    ApproxF32 {
        /// `.ftz`
        flush_to_zero: bool,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
    /// `sqrt.rnd{.ftz}.f32 d, a;`
    RndF32 {
        /// `.rn`, `.rz`, `.rm`, `.rp`
        rounding: Rounding,
        /// `.ftz`
        flush_to_zero: bool,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
    /// `sqrt.rnd.f64 d, a;`
    RndF64 {
        /// `.rn`, `.rz`, `.rm`, `.rp`
        rounding: Rounding,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
}

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
