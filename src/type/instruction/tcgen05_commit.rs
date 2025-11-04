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

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierArriveOne, // .mbarrier::arrive::one
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Multicast {
        MulticastCluster, // .multicast::cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
        pub commit: (), // .commit
        pub cta_group: CtaGroup, // .cta_group
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub shared_cluster: bool, // {.shared::cluster}
        pub multicast: Option<Multicast>, // {.multicast}
        pub b64: (), // .b64
        pub mbar: AddressOperand, // [mbar]
        pub ctamask: Option<Operand>, // {, ctaMask}
    }

}
