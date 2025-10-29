use crate::r#type::common::RegisterOperand;

/// `shf.direction.mode.type  d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shf {
    /// `.direction`
    pub direction: Direction,
    /// `.mode`
    pub mode: Mode,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
    /// `c`
    pub c: RegisterOperand,
}

/// `.direction = { .l, .r };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// `.l` (left shift)
    Left,
    /// `.r` (right shift)
    Right,
}

/// `.mode = { .clamp, .wrap };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.clamp`
    Clamp,
    /// `.wrap`
    Wrap,
}

/// `.type = { .b32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
}
