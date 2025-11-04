//! Original PTX specification:
//!
//! tcgen05.shift.cta_group.down  [taddr];
//! .cta_group = { .cta_group::1, .cta_group::2 }

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05ShiftCtaGroupDown {
        pub shift: (), // .shift
        pub cta_group: CtaGroup, // .cta_group
        pub down: (), // .down
        pub taddr: AddressOperand, // [taddr]
    }

}
