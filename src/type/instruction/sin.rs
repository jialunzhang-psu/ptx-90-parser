use crate::r#type::common::RegisterOperand;

/// `sin.approx{.ftz}.f32 d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sin {
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `.f32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f32`
    F32,
}
