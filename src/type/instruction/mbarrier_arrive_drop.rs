//! Original PTX specification:
//!
//! mbarrier.arrive_drop{.sem}{.scope}{.state}.b64 state,           [addr]{, count};
//! mbarrier.arrive_drop{.sem}{.scope}{.shared::cluster}.b64           _,   [addr] {,count};
//! mbarrier.arrive_drop.expect_tx{.state}{.sem}{.scope}.b64 state, [addr], tx_count;
//! mbarrier.arrive_drop.expect_tx{.shared::cluster}{.sem}{.scope}.b64   _, [addr], tx_count;
//! mbarrier.arrive_drop.noComplete{.release}{.cta}{.state}.b64 state,  [addr], count;
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
    pub struct MbarrierArriveDropSemScopeStateB64 {
        pub arrive_drop: (), // .arrive_drop
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
    pub struct MbarrierArriveDropSemScopeSharedClusterB64 {
        pub arrive_drop: (), // .arrive_drop
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
    pub struct MbarrierArriveDropExpectTxStateSemScopeB64 {
        pub arrive_drop: (), // .arrive_drop
        pub expect_tx: (), // .expect_tx
        pub state: Option<State>, // {.state}
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub b64: (), // .b64
        pub state2: GeneralOperand, // state
        pub addr: AddressOperand, // [addr]
        pub tx_count: GeneralOperand, // tx_count
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveDropExpectTxSharedClusterSemScopeB64 {
        pub arrive_drop: (), // .arrive_drop
        pub expect_tx: (), // .expect_tx
        pub shared_cluster: bool, // {.shared::cluster}
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub b64: (), // .b64
        pub operand: GeneralOperand, // _
        pub addr: AddressOperand, // [addr]
        pub tx_count: GeneralOperand, // tx_count
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct MbarrierArriveDropNocompleteReleaseCtaStateB64 {
        pub arrive_drop: (), // .arrive_drop
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
pub use section_0::MbarrierArriveDropSemScopeStateB64;
pub use section_0::MbarrierArriveDropSemScopeSharedClusterB64;
pub use section_0::MbarrierArriveDropExpectTxStateSemScopeB64;
pub use section_0::MbarrierArriveDropExpectTxSharedClusterSemScopeB64;
pub use section_0::MbarrierArriveDropNocompleteReleaseCtaStateB64;
pub use section_0::Sem as Sem0;
pub use section_0::Scope as Scope0;
pub use section_0::State as State0;
