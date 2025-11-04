//! Original PTX specification:
//!
//! mbarrier.expect_tx{.sem}{.scope}{.space}.b64 [addr], txCount;
//! .sem   = { .relaxed };
//! .scope = { .cta, .cluster };
//! .space = { .shared, .shared::cta, .shared::cluster };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cta, // .cta
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        Shared, // .shared
        SharedCta, // .shared::cta
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MbarrierExpectTxSemScopeSpaceB64 {
        pub expect_tx: (), // .expect_tx
        pub sem: Option<Sem>, // {.sem}
        pub scope: Option<Scope>, // {.scope}
        pub space: Option<Space>, // {.space}
        pub b64: (), // .b64
        pub addr: AddressOperand, // [addr]
        pub txcount: Operand, // txCount
    }

}
