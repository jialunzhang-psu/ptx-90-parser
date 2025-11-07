//! Original PTX specification:
//!
//! // convert const, global, local, or shared address to generic address
//! cvta.space.size  p, a;        // source address in register a
//! // cvta.space.size  p, var;      // get generic address of var
//! // cvta.space.size  p, var+imm;  // generic address of var+offset
//! // convert generic address to const, global, local, or shared address
//! cvta.to.space.size  p, a;
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };
//! .size  = { .u32, .u64 };

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
    pub enum Size {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtaSpaceSize {
        pub space: Space, // .space
        pub size: Size, // .size
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CvtaToSpaceSize {
        pub to: (), // .to
        pub space: Space, // .space
        pub size: Size, // .size
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
    }

}
