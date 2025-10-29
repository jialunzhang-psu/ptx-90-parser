use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];`
/// `ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt r, [p];`
/// `ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ldmatrix {
    /// `ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];`
    Standard(Standard),
    /// `ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt r, [p];`
    M8N16(M8N16),
    /// `ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];`
    M16N16(M16N16),
}

/// `ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standard {
    /// `.shape`
    pub shape: Shape,
    /// `.num`
    pub num: Num,
    /// `.trans`
    pub trans: bool,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.type`
    pub data_type: DataType,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
}

/// `ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt r, [p];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8N16 {
    /// `.num`
    pub num: Num,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.dst_fmt`
    pub destination_format: DestinationFormat,
    /// `.src_fmt`
    pub source_format: SourceFormat,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
}

/// `ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M16N16 {
    /// `.num`
    pub num: Num,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.dst_fmt`
    pub destination_format: DestinationFormat,
    /// `.src_fmt`
    pub source_format: SourceFormat,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
}

/// `.shape = {.m8n8, .m16n16};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `.m8n8`
    M8N8,
    /// `.m16n16`
    M16N16,
}

/// `.num = {.x1, .x2, .x4};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Num {
    /// `.x1`
    X1,
    /// `.x2`
    X2,
    /// `.x4`
    X4,
}

/// `.ss = {.shared{::cta}};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.shared`
    Shared,
    /// `.shared::cta`
    SharedCta,
}

/// `.type = {.b16, .b8};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b16`
    B16,
    /// `.b8`
    B8,
}

/// `.dst_fmt = { .b8x16 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationFormat {
    /// `.b8x16`
    B8x16,
}

/// `.src_fmt = { .b6x16_p32, .b4x16_p64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFormat {
    /// `.b6x16_p32`
    B6x16P32,
    /// `.b4x16_p64`
    B4x16P64,
}
