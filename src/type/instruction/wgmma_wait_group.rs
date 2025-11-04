//! Original PTX specification:
//!
//! wgmma.wait_group.sync.aligned N;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct WgmmaWaitGroupSyncAligned {
        pub wait_group: (), // .wait_group
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub n: Operand, // N
    }

}
