use crate::r#type::common::{Operand, RegisterOperand};

/// `bmsk.mode.b32 d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bmsk {
    /// `.mode`
    pub mode: Mode,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: Operand,
    /// `b`
    pub b: Operand,
}

/// `.mode = { .clamp, .wrap };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.clamp`
    Clamp,
    /// `.wrap`
    Wrap,
}
