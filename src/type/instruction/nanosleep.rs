use crate::r#type::common::*;

/// `nanosleep.u32 t;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nanosleep {
    /// `.u32`
    pub data_type: DataType,
    /// `t`
    pub delay: Operand,
}

/// `.u32`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
}
