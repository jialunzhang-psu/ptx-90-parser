use crate::r#type::common::Label;

/// `bra{.uni} tgt;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bra {
    /// `.uni`
    pub uniform: bool,
    /// `tgt`
    pub target: Label,
}
