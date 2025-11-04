//! Original PTX specification:
//!
//! sub.type       d, a, b;
//! sub{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! sub{.rnd}{.ftz}.f32x2      d, a, b;
//! sub{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! sub{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! sub{.rnd}.bf16   d, a, b;
//! sub{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! sub{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubSatS32 {
        pub sat: bool, // {.sat}
        pub s32: (), // .s32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF32 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzF32x2 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub ftz: bool, // {.ftz}
        pub f32x2: (), // .f32x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndF64 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF16 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF16x2 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndBf16 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndBf16x2 {
        pub rnd: Option<Rnd>, // {.rnd}
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
    }

}

pub mod section_3 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        F16, // .f16
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndSatF32Atype {
        pub rnd: Option<Rnd>, // {.rnd}
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub atype: Atype, // .atype
        pub d: Operand, // d
        pub a: Operand, // a
        pub c: Operand, // c
    }

}
