//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype.mode [a, b];  // unformatted
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d };
//! .cop   = { .ca, .cg, .cs, .cv };               // cache operation
//! .vec   = { none, .v2, .v4 };
//! .dtype = { .b8 , .b16, .b32, .b64 };
//! .mode = { .trap, .clamp, .zero };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
        A1d, // .a1d
        A2d, // .a2d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cop {
        Ca, // .ca
        Cg, // .cg
        Cs, // .cs
        Cv, // .cv
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        None, // none
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dtype {
        B8, // .b8
        B16, // .b16
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Trap, // .trap
        Clamp, // .clamp
        Zero, // .zero
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SuldBGeomCopVecDtypeMode {
        pub b: (), // .b
        pub geom: Geom, // .geom
        pub cop: Option<Cop>, // {.cop}
        pub vec: Vec, // .vec
        pub dtype: Dtype, // .dtype
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
    }

}
