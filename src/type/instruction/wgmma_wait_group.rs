//! Original PTX specification:
//!
//! wgmma.wait_group.sync.aligned N;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct WgmmaWaitGroupSyncAligned {
        pub wait_group: (), // .wait_group
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub n: GeneralOperand, // N
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::WgmmaWaitGroupSyncAligned;
