//! Original PTX specification:
//!
//! tcgen05.fence::before_thread_sync ;
//! tcgen05.fence::after_thread_sync  ;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05FenceBeforeThreadSync {
        pub fence_before_thread_sync: (), // .fence::before_thread_sync
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05FenceAfterThreadSync {
        pub fence_after_thread_sync: (), // .fence::after_thread_sync
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Tcgen05FenceBeforeThreadSync;
pub use section_0::Tcgen05FenceAfterThreadSync;
