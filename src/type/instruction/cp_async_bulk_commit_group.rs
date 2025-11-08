//! Original PTX specification:
//!
//! cp.async.bulk.commit_group;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncBulkCommitGroup {
        pub async_: (),       // .async
        pub bulk: (),         // .bulk
        pub commit_group: (), // .commit_group
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpAsyncBulkCommitGroup;
