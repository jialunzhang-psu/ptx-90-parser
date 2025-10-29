use crate::r#type::common::RegisterOperand;

/// `activemask.b32 d;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activemask {
    /// `d`
    pub destination: RegisterOperand,
}
