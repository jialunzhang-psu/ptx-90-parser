//! Original PTX specification:
//!
//! fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
//! fma.rnd{.ftz}.f32x2      d, a, b, c;
//! fma.rnd.f64              d, a, b, c;
//! .rnd = { .rn, .rz, .rm, .rp };
//! ---------------------------------------------
//! fma.rnd{.ftz}{.sat}.f16     d, a, b, c;
//! fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
//! fma.rnd{.ftz}.relu.f16      d, a, b, c;
//! fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
//! fma.rnd{.relu}.bf16         d, a, b, c;
//! fma.rnd{.relu}.bf16x2       d, a, b, c;
//! fma.rnd.oob{.relu}.type     d, a, b, c;
//! .rnd = { .rn };
//! ---------------------------------------------
//! fma.rnd{.sat}.f32.abtype  d, a, b, c;
//! .abtype = { .f16, .bf16};
//! .rnd    = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzSatF32 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzF32x2 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub f32x2: (), // .f32x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndF64 {
        pub rnd: Rnd, // .rnd
        pub f64: (), // .f64
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzSatF16 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzSatF16x2 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub sat: bool, // {.sat}
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzReluF16 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub relu: (), // .relu
        pub f16: (), // .f16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndFtzReluF16x2 {
        pub rnd: Rnd, // .rnd
        pub ftz: bool, // {.ftz}
        pub relu: (), // .relu
        pub f16x2: (), // .f16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndReluBf16 {
        pub rnd: Rnd, // .rnd
        pub relu: bool, // {.relu}
        pub bf16: (), // .bf16
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndReluBf16x2 {
        pub rnd: Rnd, // .rnd
        pub relu: bool, // {.relu}
        pub bf16x2: (), // .bf16x2
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndOobReluType {
        pub rnd: Rnd, // .rnd
        pub oob: (), // .oob
        pub relu: bool, // {.relu}
        pub type_: (), // .type
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Abtype {
        F16, // .f16
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FmaRndSatF32Abtype {
        pub rnd: Rnd, // .rnd
        pub sat: bool, // {.sat}
        pub f32: (), // .f32
        pub abtype: Abtype, // .abtype
        pub d: Operand, // d
        pub a: Operand, // a
        pub b: Operand, // b
        pub c: Operand, // c
    }

}
