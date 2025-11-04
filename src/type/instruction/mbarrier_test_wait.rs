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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Acquire, // .acquire
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
    pub struct MbarrierTestWaitSemScopeStateB64 {
        pub test_wait: (), // .test_wait
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: Operand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub state2: Operand, // state
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierTestWaitParitySemScopeStateB64 {
        pub test_wait: (), // .test_wait
        pub parity: (), // .parity
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: Operand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub phaseparity: Operand, // phaseParity
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierTryWaitSemScopeStateB64 {
        pub try_wait: (), // .try_wait
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: Operand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub state2: Operand, // state
        pub suspendtimehint: Option<Operand>, // {, suspendTimeHint}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierTryWaitParitySemScopeStateB64 {
        pub try_wait: (), // .try_wait
        pub parity: (), // .parity
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub waitcomplete: Operand, // waitComplete
        pub addr: AddressOperand, // [addr]
        pub phaseparity: Operand, // phaseParity
        pub suspendtimehint: Option<Operand>, // {, suspendTimeHint}
    }

}
