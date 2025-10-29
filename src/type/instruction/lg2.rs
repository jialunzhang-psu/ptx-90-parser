use crate::r#type::common::RegisterOperand;

/// `lg2.approx{.ftz}.f32  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lg2 {
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `.f32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.f32`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f32`
    F32,
}
