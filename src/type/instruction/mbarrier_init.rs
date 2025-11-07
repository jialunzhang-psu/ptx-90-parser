//! Original PTX specification:
//!
//! mbarrier.init{.state}.b64 [addr], count;
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
    pub struct MbarrierInitStateB64 {
        pub init: (), // .init
        pub state: Option<State>, // {.state}
        pub b64: (), // .b64
        pub addr: AddressOperand, // [addr]
        pub count: GeneralOperand, // count
    }

}
