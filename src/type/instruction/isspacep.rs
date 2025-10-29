use crate::r#type::common::{PredicateRegister, RegisterOperand};

/// `isspacep.space  p, a;    // result is .pred`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isspacep {
    /// `.space`
    pub space: Space,
    /// `p`
    pub predicate: PredicateRegister,
    /// `a`
    pub address: RegisterOperand,
}

/// `.space = { const, .global, .local, .shared{::cta, ::cluster}, .param{::entry} };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// `const`
    Const,
    /// `.global`
    Global,
    /// `.local`
    Local,
    /// `.shared`
    Shared,
    /// `.shared::cta`
    SharedCta,
    /// `.shared::cluster`
    SharedCluster,
    /// `.param`
    Param,
    /// `.param::entry`
    ParamEntry,
}
