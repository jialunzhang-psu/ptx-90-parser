use crate::r#type::common::RegisterOperand;

/// `copysign.type  d, a, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copysign {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: RegisterOperand,
}

/// `.type = { .f32, .f64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}
