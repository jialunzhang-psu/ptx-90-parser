//! Original PTX specification:
//!
//! cp.async.commit_group ;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncCommitGroup {
        pub async_: (), // .async
        pub commit_group: (), // .commit_group
    }

}
