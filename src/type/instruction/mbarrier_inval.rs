//! Original PTX specification:
//!
//! mbarrier.inval{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        SharedCta, // .shared::cta
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct MbarrierInvalStateB64 {
        pub inval: (),            // .inval
        pub state: Option<State>, // {.state}
        pub b64: (),              // .b64
        pub addr: AddressOperand, // [addr]
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierInvalStateB64;
pub use section_0::State as State0;
