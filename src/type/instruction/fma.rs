use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fma {
    /// `fma.rnd{.ftz}{.sat}.f32 d, a, b, c;`
    F32 {
        /// `.rnd`
        rounding: Rounding,
        /// `.ftz`
        flush_to_zero: bool,
        /// `.sat`
        saturate: bool,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        multiplicand_a: RegisterOperand,
        /// `b`
        multiplicand_b: RegisterOperand,
        /// `c`
        addend: RegisterOperand,
    },
    /// `fma.rnd{.ftz}.f32x2 d, a, b, c;`
    F32x2 {
        /// `.rnd`
        rounding: Rounding,
        /// `.ftz`
        flush_to_zero: bool,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        multiplicand_a: RegisterOperand,
        /// `b`
        multiplicand_b: RegisterOperand,
        /// `c`
        addend: RegisterOperand,
    },
    /// `fma.rnd.f64 d, a, b, c;`
    F64 {
        /// `.rnd`
        rounding: Rounding,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        multiplicand_a: RegisterOperand,
        /// `b`
        multiplicand_b: RegisterOperand,
        /// `c`
        addend: RegisterOperand,
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
