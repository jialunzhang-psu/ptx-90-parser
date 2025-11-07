//! Original PTX specification:
//!
//! mbarrier.inval{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum State {
        SharedCta, // .shared::cta
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierInvalStateB64 {
        pub inval: (), // .inval
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub addr: AddressOperand, // [addr]
    }

}
