//! Original PTX specification:
//!
//! cp.async.wait_group N;
//! cp.async.wait_all ;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncWaitGroup {
        pub async_: (), // .async
        pub wait_group: (), // .wait_group
        pub n: Operand, // N
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncWaitAll {
        pub async_: (), // .async
        pub wait_all: (), // .wait_all
    }

}
