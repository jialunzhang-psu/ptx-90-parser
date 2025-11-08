//! Original PTX specification:
//!
//! mbarrier.pending_count.b64 count, state;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierPendingCountB64 {
        pub pending_count: (),     // .pending_count
        pub b64: (),               // .b64
        pub count: GeneralOperand, // count
        pub state: GeneralOperand, // state
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierPendingCountB64;
