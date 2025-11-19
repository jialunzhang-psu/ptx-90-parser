//! Original PTX specification:
//!
//! mbarrier.complete_tx{.sem}{.scope}{.space}.b64 [addr], txCount;
//! .sem   = { .relaxed };
//! .scope = { .cta, .cluster };
//! .space = { .shared, .shared::cta, .shared::cluster };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        SharedCluster, // .shared::cluster
        SharedCta,     // .shared::cta
        Shared,        // .shared
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct MbarrierCompleteTxSemScopeSpaceB64 {
        pub complete_tx: (),         // .complete_tx
        pub sem: Option<Sem>,        // {.sem}
        pub scope: Option<Scope>,    // {.scope}
        pub space: Option<Space>,    // {.space}
        pub b64: (),                 // .b64
        pub addr: AddressOperand,    // [addr]
        pub txcount: GeneralOperand, // txCount
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::MbarrierCompleteTxSemScopeSpaceB64;
pub use section_0::Scope as Scope0;
pub use section_0::Sem as Sem0;
pub use section_0::Space as Space0;
