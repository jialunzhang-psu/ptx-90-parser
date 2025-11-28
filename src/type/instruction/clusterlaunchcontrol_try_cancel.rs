//! Original PTX specification:
//!
//! clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .space = { .shared::cta };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Space {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 {
        pub try_cancel: (),                            // .try_cancel
        pub async_: (),                                // .async
        pub space: Option<Space>,                      // {.space}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub multicast_cluster_all: bool,               // {.multicast::cluster::all}
        pub b128: (),                                  // .b128
        pub addr: AddressOperand,                      // [addr]
        pub mbar: AddressOperand,                      // [mbar]
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128;
pub use section_0::CompletionMechanism as CompletionMechanism0;
pub use section_0::Space as Space0;
