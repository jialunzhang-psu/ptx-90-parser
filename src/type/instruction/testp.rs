use crate::r#type::common::{PredicateRegister, RegisterOperand};

/// `testp.op.type  p, a;  // result is .pred`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Testp {
    /// `.op`
    pub test: PredicateTest,
    /// `.type`
    pub data_type: DataType,
    /// `p`
    pub destination: PredicateRegister,
    /// `a`
    pub source: RegisterOperand,
}

/// `.op = { .finite, .infinite, .number, .notanumber, .normal, .subnormal };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateTest {
    /// `.finite`
    Finite,
    /// `.infinite`
    Infinite,
    /// `.number`
    Number,
    /// `.notanumber`
    NotANumber,
    /// `.normal`
    Normal,
    /// `.subnormal`
    Subnormal,
}

/// `.type = { .f32, .f64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}
