use crate::r#type::common::{Label, RegisterOperand};

/// `brx.idx{.uni} index, tlist;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brx {
    /// `.uni`
    pub uniform: bool,
    /// `index`
    pub index: RegisterOperand,
    /// `tlist`
    pub targets: Label,
}
