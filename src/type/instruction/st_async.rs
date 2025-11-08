//! Original PTX specification:
//!
//! st.async{.sem}{.scope}{.ss}{.completion_mechanism}{.vec}.type [a], b, [mbar];
//! .sem  =                 { .weak };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .type =                 { .b32, .b64,
//! .u32, .u64,
//! .s32, .s64,
//! .f32, .f64 };
//! .vec  =                 { .v2, .v4 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ---------------------------------------------------------
//! st.async{.mmio}.sem.scope{.ss}.type [a], b;
//! .sem =                  { .release };
//! .scope =                { .gpu, .sys };
//! .ss =                   { .global };
//! .type =                 { .b8, .b16, .b32, .b64,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Weak, // .weak
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Vec {
        V2, // .v2
        V4, // .v4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
        U32, // .u32
        U64, // .u64
        S32, // .s32
        S64, // .s64
        F32, // .f32
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StAsyncSemScopeSsCompletionMechanismVecType {
        pub async_: (),                                        // .async
        pub sem: Option<Sem>,                                  // {.sem}
        pub scope: Option<Scope>,                              // {.scope}
        pub ss: Option<Ss>,                                    // {.ss}
        pub completion_mechanism: Option<CompletionMechanism>, // {.completion_mechanism}
        pub vec: Option<Vec>,                                  // {.vec}
        pub type_: Type,                                       // .type
        pub a: AddressOperand,                                 // [a]
        pub b: GeneralOperand,                                 // b
        pub mbar: AddressOperand,                              // [mbar]
    }
}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Release, // .release
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Gpu, // .gpu
        Sys, // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
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
        B8,  // .b8
        U8,  // .u8
        S8,  // .s8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StAsyncMmioSemScopeSsType {
        pub async_: (),        // .async
        pub mmio: bool,        // {.mmio}
        pub sem: Sem,          // .sem
        pub scope: Scope,      // .scope
        pub ss: Option<Ss>,    // {.ss}
        pub type_: Type,       // .type
        pub a: AddressOperand, // [a]
        pub b: GeneralOperand, // b
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CompletionMechanism as CompletionMechanism0;
pub use section_0::Scope as Scope0;
pub use section_0::Sem as Sem0;
pub use section_0::Ss as Ss0;
pub use section_0::StAsyncSemScopeSsCompletionMechanismVecType;
pub use section_0::Type as Type0;
pub use section_0::Vec as Vec0;
pub use section_1::Scope as Scope1;
pub use section_1::Sem as Sem1;
pub use section_1::Ss as Ss1;
pub use section_1::StAsyncMmioSemScopeSsType;
pub use section_1::Type as Type1;
