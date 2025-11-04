//! Original PTX specification:
//!
//! tcgen05.fence::before_thread_sync ;
//! tcgen05.fence::after_thread_sync  ;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05FenceBeforeThreadSync {
        pub fence_before_thread_sync: (), // .fence::before_thread_sync
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05FenceAfterThreadSync {
        pub fence_after_thread_sync: (), // .fence::after_thread_sync
    }

}
