//! Original PTX specification:
//!
//! clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .space = { .shared::cta };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Space {
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CompletionMechanism {
        MbarrierCompleteTxBytes, // .mbarrier::complete_tx::bytes
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 {
        pub try_cancel: (), // .try_cancel
        pub async_: (), // .async
        pub space: Option<Space>, // {.space}
        pub completion_mechanism: CompletionMechanism, // .completion_mechanism
        pub multicast_cluster_all: bool, // {.multicast::cluster::all}
        pub b128: (), // .b128
        pub addr: AddressOperand, // [addr]
        pub mbar: AddressOperand, // [mbar]
    }

}
