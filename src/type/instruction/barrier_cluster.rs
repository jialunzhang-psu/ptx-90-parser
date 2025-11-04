//! Original PTX specification:
//!
//! barrier.cluster.arrive{.sem}{.aligned};
//! barrier.cluster.wait{.acquire}{.aligned};
//! .sem = {.release, .relaxed};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        Release, // .release
        Relaxed, // .relaxed
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierClusterArriveSemAligned {
        pub cluster: (), // .cluster
        pub arrive: (), // .arrive
        pub sem: Option<Sem>, // {.sem}
        pub aligned: bool, // {.aligned}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierClusterWaitAcquireAligned {
        pub cluster: (), // .cluster
        pub wait: (), // .wait
        pub acquire: bool, // {.acquire}
        pub aligned: bool, // {.aligned}
    }

}
