//! Original PTX specification:
//!
//! isspacep.space  p, a;    // result is .pred
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        Const, // .const
        Global, // .global
        Local, // .local
        Shared, // .shared
        SharedCta, // .shared::cta
        SharedCluster, // .shared::cluster
        Param, // .param
        ParamEntry, // .param::entry
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct IsspacepSpace {
        pub space: Space, // .space
        pub p: Operand, // p
        pub a: Operand, // a
    }

}
