//! Original PTX specification:
//!
//! cp.async.bulk.wait_group{.read} N;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkWaitGroupRead {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub wait_group: (), // .wait_group
        pub read: bool, // {.read}
        pub n: Operand, // N
    }

}
