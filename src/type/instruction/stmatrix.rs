//! Original PTX specification:
//!
//! stmatrix.sync.aligned.shape.num{.trans}{.ss}.type [p], r;
//! .shape  = {.m8n8, .m16n8};
//! .num    = {.x1, .x2, .x4};
//! .ss     = {.shared, .shared::cta};
//! .type   = {.b16, .b8};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M16n8, // .m16n8
        M8n8, // .m8n8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Num {
        X1, // .x1
        X2, // .x2
        X4, // .x4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
        B8, // .b8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StmatrixSyncAlignedShapeNumTransSsType {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub num: Num, // .num
        pub trans: bool, // {.trans}
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub p: AddressOperand, // [p]
        pub r: GeneralOperand, // r
    }

}
