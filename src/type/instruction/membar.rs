//! Original PTX specification:
//!
//! // Thread fence:
//! fence{.sem}.scope;
//! // Thread fence (uni-directional):
//! fence.acquire.sync_restrict::shared::cluster.cluster;
//! fence.release.sync_restrict::shared::cta.cluster;
//! // Operation fence (uni-directional):
//! fence.op_restrict.release.cluster;
//! // Proxy fence (bi-directional):
//! fence.proxy.proxykind;
//! // Proxy fence (uni-directional):
//! fence.proxy.to_proxykind::from_proxykind.release.scope;
//! fence.proxy.to_proxykind::from_proxykind.acquire.scope  [addr], size;
//! fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;
//! fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;
//! // Old style membar:
//! membar.level;
//! membar.proxy.proxykind;
//! .sem       = { .sc, .acq_rel, .acquire, .release };
//! .scope     = { .cta, .cluster, .gpu, .sys };
//! .level     = { .cta, .gl, .sys };
//! .proxykind = { .alias, .async, .async.global, .async.shared::cta, .async.shared::cluster};
//! .op_restrict = { .mbarrier_init };
//! .to_proxykind::from_proxykind = {.tensormap::generic};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Sem {
        AcqRel,  // .acq_rel
        Acquire, // .acquire
        Release, // .release
        Sc,      // .sc
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
        Gpu,     // .gpu
        Sys,     // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum OpRestrict {
        MbarrierInit, // .mbarrier_init
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Proxykind {
        AsyncSharedCluster, // .async.shared::cluster
        AsyncSharedCta,     // .async.shared::cta
        AsyncGlobal,        // .async.global
        Alias,              // .alias
        Async,              // .async
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ToProxykindFromProxykind {
        TensormapGeneric, // .tensormap::generic
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Level {
        Cta, // .cta
        Sys, // .sys
        Gl,  // .gl
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceSemScope {
        pub sem: Option<Sem>, // {.sem}
        pub scope: Scope,     // .scope
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceAcquireSyncRestrictSharedClusterCluster {
        pub acquire: (),                      // .acquire
        pub sync_restrict_shared_cluster: (), // .sync_restrict::shared::cluster
        pub cluster: (),                      // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceReleaseSyncRestrictSharedCtaCluster {
        pub release: (),                  // .release
        pub sync_restrict_shared_cta: (), // .sync_restrict::shared::cta
        pub cluster: (),                  // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceOpRestrictReleaseCluster {
        pub op_restrict: OpRestrict, // .op_restrict
        pub release: (),             // .release
        pub cluster: (),             // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceProxyProxykind {
        pub proxy: (),            // .proxy
        pub proxykind: Proxykind, // .proxykind
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceProxyToProxykindFromProxykindReleaseScope {
        pub proxy: (),                                             // .proxy
        pub to_proxykind_from_proxykind: ToProxykindFromProxykind, // .to_proxykind::from_proxykind
        pub release: (),                                           // .release
        pub scope: Scope,                                          // .scope
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceProxyToProxykindFromProxykindAcquireScope {
        pub proxy: (),                                             // .proxy
        pub to_proxykind_from_proxykind: ToProxykindFromProxykind, // .to_proxykind::from_proxykind
        pub acquire: (),                                           // .acquire
        pub scope: Scope,                                          // .scope
        pub addr: AddressOperand,                                  // [addr]
        pub size: GeneralOperand,                                  // size
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
        pub proxy: (),                        // .proxy
        pub async_generic: (),                // .async::generic
        pub acquire: (),                      // .acquire
        pub sync_restrict_shared_cluster: (), // .sync_restrict::shared::cluster
        pub cluster: (),                      // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
        pub proxy: (),                    // .proxy
        pub async_generic: (),            // .async::generic
        pub release: (),                  // .release
        pub sync_restrict_shared_cta: (), // .sync_restrict::shared::cta
        pub cluster: (),                  // .cluster
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MembarLevel {
        pub level: Level, // .level
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MembarProxyProxykind {
        pub proxy: (),            // .proxy
        pub proxykind: Proxykind, // .proxykind
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::FenceAcquireSyncRestrictSharedClusterCluster;
pub use section_0::FenceOpRestrictReleaseCluster;
pub use section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster;
pub use section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster;
pub use section_0::FenceProxyProxykind;
pub use section_0::FenceProxyToProxykindFromProxykindAcquireScope;
pub use section_0::FenceProxyToProxykindFromProxykindReleaseScope;
pub use section_0::FenceReleaseSyncRestrictSharedCtaCluster;
pub use section_0::FenceSemScope;
pub use section_0::Level as Level0;
pub use section_0::MembarLevel;
pub use section_0::MembarProxyProxykind;
pub use section_0::OpRestrict as OpRestrict0;
pub use section_0::Proxykind as Proxykind0;
pub use section_0::Scope as Scope0;
pub use section_0::Sem as Sem0;
pub use section_0::ToProxykindFromProxykind as ToProxykindFromProxykind0;
