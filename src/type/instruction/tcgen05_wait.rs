//! Original PTX specification:
//!
//! tcgen05.wait_operation.sync.aligned;
//! .wait_operation = { .wait::ld, .wait::st }

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum WaitOperation {
        WaitLd, // .wait::ld
        WaitSt, // .wait::st
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05WaitOperationSyncAligned {
        pub wait_operation: WaitOperation, // .wait_operation
        pub sync: (), // .sync
        pub aligned: (), // .aligned
    }

}
