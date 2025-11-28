//! Original PTX specification:
//!
//! // convert const, global, local, or shared address to generic address
//! cvta.space.size  p, a;        // source address in register a
//! // cvta.space.size  p, var;      // get generic address of var
//! // cvta.space.size  p, var+imm;  // generic address of var+offset
//! // convert generic address to const, global, local, or shared address
//! cvta.to.space.size  p, a;
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };
//! .size  = { .u32, .u64 };

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
        ParamEntry,    // .param::entry
        SharedCta,     // .shared::cta
        Global,        // .global
        Shared,        // .shared
        Const,         // .const
        Local,         // .local
        Param,         // .param
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Size {
        U32, // .u32
        U64, // .u64
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CvtaSpaceSize {
        pub space: Space,      // .space
        pub size: Size,        // .size
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct CvtaToSpaceSize {
        pub to: (),            // .to
        pub space: Space,      // .space
        pub size: Size,        // .size
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CvtaSpaceSize;
pub use section_0::CvtaToSpaceSize;
pub use section_0::Size as Size0;
pub use section_0::Space as Space0;
