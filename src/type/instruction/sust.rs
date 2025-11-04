//! Original PTX specification:
//!
//! sust.b.dim{.cop}.vec.ctype.mode [a, b], c;  // unformatted
//! sust.p.dim.vec.b32.mode       [a, b], c;  // formatted
//! sust.b.adim{.cop}.vec.ctype.mode   [a, b], c;  // unformatted
//! .cop   = { .wb, .cg, .cs, .wt };                     // cache operation
//! .vec   = { none, .v2, .v4 };
//! .ctype = { .b8 , .b16, .b32, .b64 };
//! .mode  = { .trap, .clamp, .zero };
//! .dim   = { .1d, .2d, .3d };
//! .adim  = { .a1d, .a2d };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Dim {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Cop {
        Wb, // .wb
        Cg, // .cg
        Cs, // .cs
        Wt, // .wt
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        None, // none
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
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
    pub enum Adim {
        A1d, // .a1d
        A2d, // .a2d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SustBDimCopVecCtypeMode {
        pub b: (), // .b
        pub dim: Dim, // .dim
        pub cop: Option<Cop>, // {.cop}
        pub vec: Vec, // .vec
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SustPDimVecB32Mode {
        pub p: (), // .p
        pub dim: Dim, // .dim
        pub vec: Vec, // .vec
        pub b32: (), // .b32
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SustBAdimCopVecCtypeMode {
        pub b: (), // .b
        pub adim: Adim, // .adim
        pub cop: Option<Cop>, // {.cop}
        pub vec: Vec, // .vec
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
        pub c: Operand, // c
    }

}
