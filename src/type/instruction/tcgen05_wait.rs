//! Original PTX specification:
//!
//! tcgen05.wait_operation.sync.aligned;
//! .wait_operation = { .wait::ld, .wait::st }

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum WaitOperation {
        WaitLd, // .wait::ld
        WaitSt, // .wait::st
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05WaitOperationSyncAligned {
        pub wait_operation: WaitOperation, // .wait_operation
        pub sync: (),                      // .sync
        pub aligned: (),                   // .aligned
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Tcgen05WaitOperationSyncAligned;
pub use section_0::WaitOperation as WaitOperation0;
