//! Original PTX specification:
//!
//! ldu{.ss}.type      d, [a];       // load from address
//! ldu{.ss}.vec.type  d, [a];       // vec load from address
//! .ss   = { .global };             // state space
//! .vec  = { .v2, .v4 };
//! .type = { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B128, // .b128
        B16,  // .b16
        B32,  // .b32
        B64,  // .b64
        U16,  // .u16
        U32,  // .u32
        U64,  // .u64
        S16,  // .s16
        S32,  // .s32
        S64,  // .s64
        F32,  // .f32
        F64,  // .f64
        B8,   // .b8
        U8,   // .u8
        S8,   // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LduSsType {
        pub ss: Option<Ss>,    // {.ss}
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LduSsVecType {
        pub ss: Option<Ss>,    // {.ss}
        pub vec: Vec,          // .vec
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::LduSsType;
pub use section_0::LduSsVecType;
pub use section_0::Ss as Ss0;
pub use section_0::Type as Type0;
pub use section_0::Vec as Vec0;
