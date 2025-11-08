//! Original PTX specification:
//!
//! wgmma.commit_group.sync.aligned;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct WgmmaCommitGroupSyncAligned {
        pub commit_group: (), // .commit_group
        pub sync: (),         // .sync
        pub aligned: (),      // .aligned
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::WgmmaCommitGroupSyncAligned;
