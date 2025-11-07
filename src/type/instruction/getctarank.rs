//! Original PTX specification:
//!
//! getctarank{.space}.type d, a;
//! // Get cta rank from source shared memory address in register a.
//! getctarank.shared::cluster.type d, a;
//! // // Get cta rank from shared memory variable.
//! // getctarank.shared::cluster.type d, var;
//! // // Get cta rank from shared memory variable+offset.
//! // getctarank.shared::cluster.type d, var + imm;
//! // Get cta rank from generic address of shared memory variable in register a.
//! getctarank.type d, a;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetctarankSpaceType {
        pub space: Option<Space>, // {.space}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetctarankSharedClusterType {
        pub shared_cluster: (), // .shared::cluster
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GetctarankType {
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
