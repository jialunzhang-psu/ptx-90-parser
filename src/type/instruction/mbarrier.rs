use crate::r#type::common::{AddressOperand, Operand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mbarrier {
    /// `mbarrier.init{.shared{::cta}}.b64 [addr], count;`
    Init(Init),
}

/// `mbarrier.init{.shared{::cta}}.b64 [addr], count;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Init {
    /// `.shared{::cta}`
    pub shared_space: SharedSpace,
    /// `.b64`
    pub data_type: DataType,
    /// `[addr]`
    pub address: AddressOperand,
    /// `count`
    pub count: Operand,
}

/// `.shared{::cta}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSpace {
    /// (absent)
    Generic,
    /// `.shared`
    Shared,
    /// `.shared::cta`
    SharedCta,
}

/// `.b64`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b64`
    B64,
}
