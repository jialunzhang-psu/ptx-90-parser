use crate::r#type::common::RegisterOperand;

/// `prmt.b32{.mode} d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prmt {
    /// `.mode`
    pub mode: Option<Mode>,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `c`
    pub c: RegisterOperand,
}

/// `.mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.f4e`
    F4e,
    /// `.b4e`
    B4e,
    /// `.rc8`
    Rc8,
    /// `.ecl`
    Ecl,
    /// `.ecr`
    Ecr,
    /// `.rc16`
    Rc16,
}
