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
        pub sync: (), // .sync
        pub aligned: (), // .aligned
    }

}
