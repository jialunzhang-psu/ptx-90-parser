//! Original PTX specification:
//!
//! madc.hilo{.cc}.type  d, a, b, c;
//! .type = { .u32, .s32, .u64, .s64 };
//! .hilo = { .hi, .lo };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Hilo {
        Hi, // .hi
        Lo, // .lo
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U32, // .u32
        S32, // .s32
        U64, // .u64
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MadcHiloCcType {
        pub hilo: Hilo, // .hilo
        pub cc: bool, // {.cc}
        pub type_: Type, // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
        pub c: GeneralOperand, // c
    }

}
