use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `ldu{.ss}.type d, [a];`
/// `ldu{.ss}.vec.type d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ldu {
    /// `ldu{.ss}.type d, [a];`
    Scalar(Scalar),
    /// `ldu{.ss}.vec.type d, [a];`
    Vector(Vector),
}

/// `ldu{.ss}.type d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `[a]`
    pub address: AddressOperand,
}

/// `ldu{.ss}.vec.type d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector {
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: VectorDestination,
    /// `[a]`
    pub address: AddressOperand,
}

/// `.ss = { .global };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.global`
    Global,
}

/// `.type = { .b8, .b16, .b32, .b64, .b128, .u8, .u16, .u32, .u64, .s8, .s16, .s32, .s64, .f32, .f64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b8`
    B8,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
    /// `.b128`
    B128,
    /// `.u8`
    U8,
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s8`
    S8,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}

/// `.vec = { .v2, .v4 };`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorDestination {
    /// `.v2`
    V2([RegisterOperand; 2]),
    /// `.v4`
    V4([RegisterOperand; 4]),
}
