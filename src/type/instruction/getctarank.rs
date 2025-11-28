//! Original PTX specification:
//!
//! getctarank{.space}.type d, a;
//! // Get cta rank from source shared memory address in register a.
//! getctarank.shared::cluster.type d, a;
//! // // Get cta rank from shared memory variable.
//! // getctarank.shared::cluster.type d, var;
//! // // Get cta rank from shared memory variable+offset.
//! // getctarank.shared::cluster.type d, var + imm;
//! // Get cta rank from generic address of shared memory variable in register a.
//! getctarank.type d, a;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Space {
        SharedCluster, // .shared::cluster
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Type {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct GetctarankSpaceType {
        pub space: Option<Space>, // {.space}
        pub type_: Type,          // .type
        pub d: GeneralOperand,    // d
        pub a: GeneralOperand,    // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct GetctarankSharedClusterType {
        pub shared_cluster: (), // .shared::cluster
        pub type_: Type,        // .type
        pub d: GeneralOperand,  // d
        pub a: GeneralOperand,  // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct GetctarankType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::GetctarankSharedClusterType;
pub use section_0::GetctarankSpaceType;
pub use section_0::GetctarankType;
pub use section_0::Space as Space0;
pub use section_0::Type as Type0;
