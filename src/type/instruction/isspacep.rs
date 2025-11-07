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
        SharedCluster, // .shared::cluster
        ParamEntry, // .param::entry
        SharedCta, // .shared::cta
        Global, // .global
        Shared, // .shared
        Const, // .const
        Local, // .local
        Param, // .param
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct IsspacepSpace {
        pub space: Space, // .space
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
    }

}
