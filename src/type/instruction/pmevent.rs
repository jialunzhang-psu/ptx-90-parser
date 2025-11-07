//! Original PTX specification:
//!
//! pmevent a;         // trigger a single performance monitor event
//! pmevent.mask a;    // trigger one or more performance monitor events

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pmevent {
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PmeventMask {
        pub mask: (), // .mask
        pub a: GeneralOperand, // a
    }

}
