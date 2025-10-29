use crate::r#type::common::AddressOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceInstruction {
    /// `fence{.sem}.scope;`
    Thread(FenceThread),
    /// `fence.acquire.sync_restrict::shared::cluster.cluster;`
    /// `fence.release.sync_restrict::shared::cta.cluster;`
    ThreadSyncRestrict(FenceThreadSyncRestrict),
    /// `fence.mbarrier_init.release.cluster;`
    OperationRestrict(FenceOperationRestrict),
    /// `fence.proxy.proxykind;`
    Proxy(MembarProxy),
    /// `fence.proxy.to_proxykind::from_proxykind.release.scope;`
    ProxyTensorMapRelease(FenceProxyTensorMapRelease),
    /// `fence.proxy.to_proxykind::from_proxykind.acquire.scope  [addr], size;`
    ProxyTensorMapAcquire(FenceProxyTensorMapAcquire),
    /// `fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;`
    /// `fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;`
    ProxyAsync(FenceProxyAsync),
    /// `membar.level;`
    MembarScope(MembarLevel),
    /// `membar.proxy.proxykind;`
    MembarProxy(MembarProxy),
}

/// `fence{.sem}.scope;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceThread {
    pub semantics: Option<FenceSemantics>,
    pub scope: FenceScope,
}

/// `fence.acquire.sync_restrict::shared::cluster.cluster;`
/// `fence.release.sync_restrict::shared::cta.cluster;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceThreadSyncRestrict {
    pub semantics: FenceSyncSemantics,
    pub shared: FenceSyncRestrictShared,
    pub scope: FenceScope,
}

/// `fence.mbarrier_init.release.cluster;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceOperationRestrict {
    pub restriction: FenceOperationRestriction,
    pub semantics: FenceSyncSemantics,
    pub scope: FenceScope,
}

/// `fence.proxy.to_proxykind::from_proxykind.release.scope;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceProxyTensorMapRelease {
    pub direction: FenceTensorMapDirection,
    pub scope: FenceScope,
}

/// `fence.proxy.to_proxykind::from_proxykind.acquire.scope  [addr], size;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceProxyTensorMapAcquire {
    pub direction: FenceTensorMapDirection,
    pub scope: FenceScope,
    pub address: AddressOperand,
    pub size: FenceProxySize,
}

/// `fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;`
/// `fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceProxyAsync {
    pub semantics: FenceSyncSemantics,
    pub shared: FenceSyncRestrictShared,
    pub scope: FenceScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceSemantics {
    /// `.sc`
    Sc,
    /// `.acq_rel`
    AcqRel,
    /// `.acquire`
    Acquire,
    /// `.release`
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceScope {
    /// `.cta`
    Cta,
    /// `.cluster`
    Cluster,
    /// `.gpu`
    Gpu,
    /// `.sys`
    Sys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceSyncSemantics {
    /// `.acquire`
    Acquire,
    /// `.release`
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceSyncRestrictShared {
    /// `.shared::cluster`
    Cluster,
    /// `.shared::cta`
    Cta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceOperationRestriction {
    /// `.mbarrier_init`
    MbarrierInit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceProxySharedScope {
    /// `.cta`
    Cta,
    /// `.cluster`
    Cluster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceTensorMapDirection {
    /// `.tensormap::generic`
    TensormapFromGeneric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceProxySize {
    /// `128`
    N128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembarLevel {
    /// `.cta`
    Cta,
    /// `.gl`
    Gl,
    /// `.sys`
    Sys,
}

/// `membar.proxy.proxykind;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembarProxy {
    /// `.alias`
    Alias,
    /// `.async`
    Async,
    /// `.async.global`
    AsyncGlobal,
    /// `.async.shared::{cta, cluster}`
    AsyncShared(FenceProxySharedScope),
}
