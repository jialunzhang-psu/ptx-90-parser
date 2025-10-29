use crate::r#type::common::AddressOperand;

/// `discard{.global}.level [a], size;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discard {
    /// `{.global}`
    pub space: Option<Space>,
    /// `.level`
    pub level: Level,
    /// `[a]`
    pub address: AddressOperand,
    /// `size`
    pub size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// `.global`
    Global,
}

/// `.level = { .L2 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// `.L2`
    L2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// `128`
    Bytes128,
}
