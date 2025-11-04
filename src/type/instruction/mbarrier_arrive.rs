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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Release, // .release
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cta, // .cta
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        Shared, // .shared
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveSemScopeStateB64 {
        pub arrive: (), // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: Operand, // state
        pub addr: AddressOperand, // [addr]
        pub count: Option<Operand>, // {, count}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveSemScopeSharedClusterB64 {
        pub arrive: (), // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub shared_cluster: bool, // {.shared::cluster}
        pub b64: (), // .b64
        pub operand: Operand, // _
        pub addr: AddressOperand, // [addr]
        pub count: Option<Operand>, // {, count}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveExpectTxSemScopeStateB64 {
        pub arrive: (), // .arrive
        pub expect_tx: (), // .expect_tx
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: Operand, // state
        pub addr: AddressOperand, // [addr]
        pub txcount: Operand, // txCount
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveExpectTxSemScopeSharedClusterB64 {
        pub arrive: (), // .arrive
        pub expect_tx: (), // .expect_tx
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub shared_cluster: bool, // {.shared::cluster}
        pub b64: (), // .b64
        pub operand: Operand, // _
        pub addr: AddressOperand, // [addr]
        pub txcount: Operand, // txCount
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveNocompleteReleaseCtaStateB64 {
        pub arrive: (), // .arrive
        pub nocomplete: (), // .noComplete
        pub release: bool, // {.release}
        pub cta: bool, // {.cta}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: Operand, // state
        pub addr: AddressOperand, // [addr]
        pub count: Operand, // count
    }

}
