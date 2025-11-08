//! Original PTX specification:
//!
//! pmevent a;         // trigger a single performance monitor event
//! pmevent.mask a;    // trigger one or more performance monitor events

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pmevent {
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PmeventMask {
        pub mask: (),          // .mask
        pub a: GeneralOperand, // a
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Pmevent;
pub use section_0::PmeventMask;
