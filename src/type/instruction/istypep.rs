use crate::r#type::common::{PredicateRegister, RegisterOperand};

/// `istypep.type   p, a;  // result is .pred`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Istypep {
    /// `.type`
    pub data_type: DataType,
    /// `p`
    pub predicate: PredicateRegister,
    /// `a (.u64)`
    pub address: RegisterOperand,
}

/// `.type = { .texref, .samplerref, .surfref };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.texref`
    TexRef,
    /// `.samplerref`
    SamplerRef,
    /// `.surfref`
    SurfRef,
}
