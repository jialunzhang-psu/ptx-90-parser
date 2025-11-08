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
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU32 {
        pub u32: (),               // .u32
        pub d: GeneralOperand,     // d
        pub fname: GeneralOperand, // fname
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU64 {
        pub u64: (),               // .u64
        pub d: GeneralOperand,     // d
        pub fname: GeneralOperand, // fname
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU321 {
        pub u32: (),                // .u32
        pub d: GeneralOperand,      // d
        pub kernel: GeneralOperand, // kernel
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovU641 {
        pub u64: (),                // .u64
        pub d: GeneralOperand,      // d
        pub kernel: GeneralOperand, // kernel
    }
}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B128, // .b128
        B16,  // .b16
        B32,  // .b32
        B64,  // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovType1 {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MovType;
pub use section_0::MovU32;
pub use section_0::MovU64;
pub use section_0::MovU321;
pub use section_0::MovU641;
pub use section_0::Type as Type0;
pub use section_1::MovType1;
pub use section_1::Type as Type1;
