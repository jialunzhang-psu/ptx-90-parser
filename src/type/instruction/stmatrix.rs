use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `stmatrix.sync.aligned.shape.num{.trans}{.ss}.type [p], r;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmatrix {
    pub shape: Shape,
    pub num: Num,
    pub trans: bool,
    pub state_space: Option<StateSpace>,
    pub data_type: DataType,
    pub address: AddressOperand,
    pub source: Source,
}

/// `.shape = {.m8n8, .m16n8};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `.m8n8`
    M8N8,
    /// `.m16n8`
    M16N8,
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

/// `{r}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// `{r}`
    X1(RegisterOperand),
    /// `{r0, r1}`
    X2([RegisterOperand; 2]),
    /// `{r0, r1, r2, r3}`
    X4([RegisterOperand; 4]),
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
