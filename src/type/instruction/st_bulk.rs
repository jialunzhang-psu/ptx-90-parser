//! Original PTX specification:
//!
//! st.bulk{.weak}{.shared::cta}  [a], size, initval; // initval must be zero

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct StBulkWeakSharedCta {
        pub bulk: (), // .bulk
        pub weak: bool, // {.weak}
        pub shared_cta: bool, // {.shared::cta}
        pub a: AddressOperand, // [a]
        pub size: Operand, // size
        pub initval: Operand, // initval
    }

}
