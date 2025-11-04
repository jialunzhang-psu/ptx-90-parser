//! Original PTX specification:
//!
//! mov.type  d, a; 
//! // mov.type  d, sreg;
//! // mov.type  d, avar;       // get address of variable
//! // mov.type  d, avar+imm;   // get address of variable with offset
//! mov.u32   d, fname;      // get address of device function
//! mov.u64   d, fname;      // get address of device function
//! mov.u32   d, kernel;     // get address of entry function
//! mov.u64   d, kernel;     // get address of entry function
//! .type = { .pred,
//! .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };
//! ----------------------------------------------
//! mov.type  d, a;
//! .type = { .b16, .b32, .b64, .b128 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Pred, // .pred
        B16, // .b16
        B32, // .b32
        B64, // .b64
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovType {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU32 {
        pub u32: (), // .u32
        pub d: Operand, // d
        pub fname: Operand, // fname
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU64 {
        pub u64: (), // .u64
        pub d: Operand, // d
        pub fname: Operand, // fname
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU321 {
        pub u32: (), // .u32
        pub d: Operand, // d
        pub kernel: Operand, // kernel
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU641 {
        pub u64: (), // .u64
        pub d: Operand, // d
        pub kernel: Operand, // kernel
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B16, // .b16
        B32, // .b32
        B64, // .b64
        B128, // .b128
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovType1 {
        pub type_: Type, // .type
        pub d: Operand, // d
        pub a: Operand, // a
    }

}
