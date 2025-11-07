//! Original PTX specification:
//!
//! mad.mode.type  d, a, b, c;
//! mad.hi.sat.s32 d, a, b, c;
//! .mode = { .hi, .lo, .wide };
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! 
//! mad{.ftz}{.sat}.f32      d, a, b, c;    // .target sm_1x
//! mad.rnd{.ftz}{.sat}.f32  d, a, b, c;    // .target sm_20
//! mad.rnd.f64              d, a, b, c;    // .target sm_13 and higher
//! .rnd = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Wide, // .wide
        Hi, // .hi
        Lo, // .lo
    }

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
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadModeType {
        pub mode: Mode, // .mode
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadHiSatS32 {
        pub hi: (), // .hi
        pub sat: (), // .sat
        pub s32: (), // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadFtzSatF32 {
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadRndFtzSatF32 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadRndF64 {
        pub rnd: Rnd, // .rnd
        pub f64: (), // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
