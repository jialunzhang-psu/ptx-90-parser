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
    pub struct MbarrierArriveDropSemScopeStateB64 {
        pub arrive_drop: (), // .arrive_drop
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub state2: Operand, // state
        pub addr: AddressOperand, // [addr]
        pub count: Option<Operand>, // {, count}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveDropSemScopeSharedClusterB64 {
        pub arrive_drop: (), // .arrive_drop
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub shared_cluster: bool, // {.shared::cluster}
        pub b64: (), // .b64
        pub operand: Operand, // _
        pub addr: AddressOperand, // [addr]
        pub count: Option<Operand>, // {, count}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveDropExpectTxStateSemScopeB64 {
        pub arrive_drop: (), // .arrive_drop
        pub expect_tx: (), // .expect_tx
        pub state: Option<State>, // {.state}
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub b64: (), // .b64
        pub state2: Operand, // state
        pub addr: AddressOperand, // [addr]
        pub tx_count: Operand, // tx_count
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveDropExpectTxSharedClusterSemScopeB64 {
        pub arrive_drop: (), // .arrive_drop
        pub expect_tx: (), // .expect_tx
        pub shared_cluster: bool, // {.shared::cluster}
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub b64: (), // .b64
        pub operand: Operand, // _
        pub addr: AddressOperand, // [addr]
        pub tx_count: Operand, // tx_count
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierArriveDropNocompleteReleaseCtaStateB64 {
        pub arrive_drop: (), // .arrive_drop
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
