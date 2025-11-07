//! Original PTX specification:
//!
//! mapa{.space}.type          d, a, b;
//! // Maps shared memory address in register a into CTA b.
//! // mapa.shared::cluster.type  d, a, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh + imm, b;
//! // Maps generic address in register a into CTA b.
//! // mapa.type                  d, a, b;
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
    pub struct MapaSpaceType {
        pub space: Option<Space>, // {.space}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

}
