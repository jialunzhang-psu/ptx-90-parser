//! Original PTX specification:
//!
//! cp.async.bulk.wait_group{.read} N;

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct CpAsyncBulkWaitGroupRead {
        pub async_: (),        // .async
        pub bulk: (),          // .bulk
        pub wait_group: (),    // .wait_group
        pub read: bool,        // {.read}
        pub n: GeneralOperand, // N
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpAsyncBulkWaitGroupRead;
