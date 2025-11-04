//! Original PTX specification:
//!
//! cp.async.bulk.commit_group;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkCommitGroup {
        pub async_: (), // .async
        pub bulk: (), // .bulk
        pub commit_group: (), // .commit_group
    }

}
