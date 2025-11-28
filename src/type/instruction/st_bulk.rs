//! Original PTX specification:
//!
//! st.bulk{.weak}{.shared::cta}  [a], size, initval; // initval must be zero

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct StBulkWeakSharedCta {
        pub bulk: (), // .bulk
        pub weak: bool, // {.weak}
        pub shared_cta: bool, // {.shared::cta}
        pub a: AddressOperand, // [a]
        pub size: GeneralOperand, // size
        pub initval: GeneralOperand, // initval
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::StBulkWeakSharedCta;
