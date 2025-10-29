use crate::r#type::common::RegisterOperand;

/// `movmatrix.sync.aligned.shape.trans.type d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Movmatrix {
    /// `.shape`
    pub shape: Shape,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.shape = {.m8n8};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `.m8n8`
    M8N8,
}

/// `.type = {.b16};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b16`
    B16,
}
