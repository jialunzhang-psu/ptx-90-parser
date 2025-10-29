use crate::r#type::common::AddressOperand;

/// `prefetchu.L1 [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefetchu {
    pub level: PrefetchuLevel,
    pub address: AddressOperand,
}

/// `.L1`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchuLevel {
    L1,
}
