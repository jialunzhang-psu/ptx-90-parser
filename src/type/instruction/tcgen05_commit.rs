//! Original PTX specification:
//!
//! tcgen05.commit.cta_group.completion_mechanism{.shared::cluster}{.multicast}.b64
//! [mbar] {, ctaMask};
//! .completion_mechanism = { .mbarrier::arrive::one };
//! .cta_group            = { .cta_group::1, .cta_group::2 };
//! .multicast            = { .multicast::cluster };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierArriveOne, // .mbarrier::arrive::one
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Multicast {
        MulticastCluster, // .multicast::cluster
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
        pub commit: (), // .commit
        pub cta_group: CtaGroup, // .cta_group
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub shared_cluster: bool, // {.shared::cluster}
        pub multicast: Option<Multicast>, // {.multicast}
        pub b64: (), // .b64
        pub mbar: AddressOperand, // [mbar]
        pub ctamask: Option<GeneralOperand>, // {, ctaMask}
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64;
pub use section_0::CtaGroup as CtaGroup0;
pub use section_0::CompletionMechanism as CompletionMechanism0;
pub use section_0::Multicast as Multicast0;
