//! Original PTX specification:
//!
//! isspacep.space  p, a;    // result is .pred
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct IsspacepSpace {
        pub space: Space,      // .space
        pub p: GeneralOperand, // p
        pub a: GeneralOperand, // a
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::IsspacepSpace;
pub use section_0::Space as Space0;
