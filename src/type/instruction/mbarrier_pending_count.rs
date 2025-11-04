//! Original PTX specification:
//!
//! mbarrier.pending_count.b64 count, state;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierPendingCountB64 {
        pub pending_count: (), // .pending_count
        pub b64: (), // .b64
        pub count: Operand, // count
        pub state: Operand, // state
    }

}
