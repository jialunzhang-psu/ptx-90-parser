use crate::r#type::common::RegisterOperand;

/// `tanh.approx.f32 d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tanh {
    /// `.approx`
    pub approximation: Approximation,
    /// `.f32`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approximation {
    /// `.approx`
    Approx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f32`
    F32,
}
