//! Original PTX specification:
//!
//! cp.async.mbarrier.arrive{.noinc}{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        Shared, // .shared
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CpAsyncMbarrierArriveNoincStateB64 {
        pub async_: (), // .async
        pub mbarrier: (), // .mbarrier
        pub arrive: (), // .arrive
        pub noinc: bool, // {.noinc}
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub addr: AddressOperand, // [addr]
    }

}
