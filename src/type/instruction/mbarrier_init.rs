//! Original PTX specification:
//!
//! mbarrier.init{.state}.b64 [addr], count;
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum State {
        SharedCta, // .shared::cta
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierInitStateB64 {
        pub init: (),              // .init
        pub state: Option<State>,  // {.state}
        pub b64: (),               // .b64
        pub addr: AddressOperand,  // [addr]
        pub count: GeneralOperand, // count
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierInitStateB64;
pub use section_0::State as State0;
