//! Original PTX specification:
//!
//! mbarrier.arrive{.sem}{.scope}{.state}.b64           state, [addr]{, count};
//! mbarrier.arrive{.sem}{.scope}{.shared::cluster}.b64         _, [addr] {,count};
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.state}.b64 state, [addr], txCount;
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.shared::cluster}.b64   _, [addr], txCount;
//! mbarrier.arrive.noComplete{.release}{.cta}{.state}.b64  state, [addr], count;
//! .sem   = { .release, .relaxed };
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
        Release, // .release
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
    pub struct MbarrierArriveSemScopeStateB64 {
        pub arrive: (), // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: GeneralOperand, // state
        pub addr: AddressOperand, // [addr]
        pub count: Option<GeneralOperand>, // {, count}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveSemScopeSharedClusterB64 {
        pub arrive: (), // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub shared_cluster: bool, // {.shared::cluster}
        pub b64: (), // .b64
        pub operand: GeneralOperand, // _
        pub addr: AddressOperand, // [addr]
        pub count: Option<GeneralOperand>, // {, count}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveExpectTxSemScopeStateB64 {
        pub arrive: (), // .arrive
        pub expect_tx: (), // .expect_tx
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: GeneralOperand, // state
        pub addr: AddressOperand, // [addr]
        pub txcount: GeneralOperand, // txCount
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveExpectTxSemScopeSharedClusterB64 {
        pub arrive: (), // .arrive
        pub expect_tx: (), // .expect_tx
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub shared_cluster: bool, // {.shared::cluster}
        pub b64: (), // .b64
        pub operand: GeneralOperand, // _
        pub addr: AddressOperand, // [addr]
        pub txcount: GeneralOperand, // txCount
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveNocompleteReleaseCtaStateB64 {
        pub arrive: (), // .arrive
        pub nocomplete: (), // .noComplete
        pub release: bool, // {.release}
        pub cta: bool, // {.cta}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: GeneralOperand, // state
        pub addr: AddressOperand, // [addr]
        pub count: GeneralOperand, // count
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierArriveSemScopeStateB64;
pub use section_0::MbarrierArriveSemScopeSharedClusterB64;
pub use section_0::MbarrierArriveExpectTxSemScopeStateB64;
pub use section_0::MbarrierArriveExpectTxSemScopeSharedClusterB64;
pub use section_0::MbarrierArriveNocompleteReleaseCtaStateB64;
pub use section_0::Sem as Sem0;
pub use section_0::Scope as Scope0;
pub use section_0::State as State0;
