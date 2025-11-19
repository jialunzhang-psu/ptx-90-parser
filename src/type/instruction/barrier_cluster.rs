//! Original PTX specification:
//!
//! barrier.cluster.arrive{.sem}{.aligned};
//! barrier.cluster.wait{.acquire}{.aligned};
//! .sem = {.release, .relaxed};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Release, // .release
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct BarrierClusterArriveSemAligned {
        pub cluster: (),      // .cluster
        pub arrive: (),       // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub aligned: bool,    // {.aligned}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct BarrierClusterWaitAcquireAligned {
        pub cluster: (),   // .cluster
        pub wait: (),      // .wait
        pub acquire: bool, // {.acquire}
        pub aligned: bool, // {.aligned}
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BarrierClusterArriveSemAligned;
pub use section_0::BarrierClusterWaitAcquireAligned;
pub use section_0::Sem as Sem0;
