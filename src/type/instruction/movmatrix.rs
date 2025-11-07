//! Original PTX specification:
//!
//! movmatrix.sync.aligned.shape.trans.type d, a;
//! .shape  = {.m8n8};
//! .type   = {.b16};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8, // .m8n8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovmatrixSyncAlignedShapeTransType {
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shape: Shape, // .shape
        pub trans: (), // .trans
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

}
