//! Original PTX specification:
//!
//! mbarrier.test_wait{.sem}{.scope}{.state}.b64        waitComplete, [addr], state;
//! mbarrier.test_wait.parity{.sem}{.scope}{.state}.b64 waitComplete, [addr], phaseParity;
//! mbarrier.try_wait{.sem}{.scope}{.state}.b64         waitComplete, [addr], state {, suspendTimeHint};
//! mbarrier.try_wait.parity{.sem}{.scope}{.state}.b64  waitComplete, [addr], phaseParity {, suspendTimeHint};
//! .sem   = { .acquire, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Sem {
        Acquire, // .acquire
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Scope {
        Cluster, // .cluster
        Cta, // .cta
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum State {
        SharedCta, // .shared::cta
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierTestWaitSemScopeStateB64 {
        pub test_wait: (), // .test_wait
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: GeneralOperand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub state2: GeneralOperand, // state
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierTestWaitParitySemScopeStateB64 {
        pub test_wait: (), // .test_wait
        pub parity: (), // .parity
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: GeneralOperand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub phaseparity: GeneralOperand, // phaseParity
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierTryWaitSemScopeStateB64 {
        pub try_wait: (), // .try_wait
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: GeneralOperand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub state2: GeneralOperand, // state
        pub suspendtimehint: Option<GeneralOperand>, // {, suspendTimeHint}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierTryWaitParitySemScopeStateB64 {
        pub try_wait: (), // .try_wait
        pub parity: (), // .parity
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: GeneralOperand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub phaseparity: GeneralOperand, // phaseParity
        pub suspendtimehint: Option<GeneralOperand>, // {, suspendTimeHint}
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierTestWaitSemScopeStateB64;
pub use section_0::MbarrierTestWaitParitySemScopeStateB64;
pub use section_0::MbarrierTryWaitSemScopeStateB64;
pub use section_0::MbarrierTryWaitParitySemScopeStateB64;
pub use section_0::Sem as Sem0;
pub use section_0::Scope as Scope0;
pub use section_0::State as State0;
