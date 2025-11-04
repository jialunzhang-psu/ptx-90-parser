//! Original PTX specification:
//!
//! sured.b.op.geom.ctype.mode [a,b],c; // byte addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .u32, .u64, .s32, .b32, .s64 };  // for sured.b
//! .mode  = { .trap, .clamp, .zero };
//! ----------------------------------------------------
//! sured.p.op.geom.ctype.mode [a,b],c; // sample addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .b32, .b64 };                    // for sured.p
//! .mode  = { .trap, .clamp, .zero };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
        And, // .and
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        U32, // .u32
        U64, // .u64
        S32, // .s32
        B32, // .b32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Trap, // .trap
        Clamp, // .clamp
        Zero, // .zero
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SuredBOpGeomCtypeMode {
        pub b: (), // .b
        pub op: Op, // .op
        pub geom: Geom, // .geom
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
        pub c: Operand, // c
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Add, // .add
        Min, // .min
        Max, // .max
        And, // .and
        Or, // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Geom {
        _1d, // .1d
        _2d, // .2d
        _3d, // .3d
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
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
    pub struct SuredPOpGeomCtypeMode {
        pub p: (), // .p
        pub op: Op, // .op
        pub geom: Geom, // .geom
        pub ctype: Ctype, // .ctype
        pub mode: Mode, // .mode
        pub a: (Operand, Operand), // [a, b]
        pub c: Operand, // c
    }

}
