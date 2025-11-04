//! Original PTX specification:
//!
//! wgmma.fence.sync.aligned;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct WgmmaFenceSyncAligned {
        pub fence: (), // .fence
        pub sync: (), // .sync
        pub aligned: (), // .aligned
    }

}
