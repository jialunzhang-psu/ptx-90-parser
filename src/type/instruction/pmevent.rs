use crate::r#type::common::*;

/// `pmevent a;` | `pmevent.mask a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pmevent {
    /// `pmevent a;`
    Single {
        /// `a`
        event: Immediate,
    },
    /// `pmevent.mask a;`
    Mask {
        /// `a`
        mask: Immediate,
    },
}
