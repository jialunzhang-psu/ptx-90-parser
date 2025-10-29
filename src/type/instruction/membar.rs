use crate::r#type::common::AddressOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Membar {
    /// `fence{.sem}.scope;`
    ThreadFence(ThreadFence),
    /// `fence.acquire.sync_restrict::shared::cluster.cluster;`
    /// `fence.release.sync_restrict::shared::cta.cluster;`
    ThreadFenceSyncRestrict(ThreadFenceSyncRestrict),
    /// `fence.op_restrict.release.scope;`
    OperationFence(OperationFence),
    /// `fence.proxy.proxykind;`
    ProxyFence(ProxyFence),
    /// `fence.proxy.tensormap::generic.release.scope;`
    ProxyTensormapRelease(ProxyTensormapRelease),
    /// `fence.proxy.tensormap::generic.acquire.scope  [addr], size;`
    ProxyTensormapAcquire(ProxyTensormapAcquire),
    /// `fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;`
    /// `fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;`
    ProxyAsync(ProxyAsync),
    /// `membar.level;`
    OldStyleScope(OldStyleScope),
    /// `membar.proxy.proxykind;`
    OldStyleProxy(OldStyleProxy),
}

/// `fence{.sem}.scope;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadFence {
    /// `.sem`
    pub semantics: Option<Semantics>,
    /// `.scope`
    pub scope: Scope,
}

/// `fence.acquire.sync_restrict::shared::cluster.cluster;`
/// `fence.release.sync_restrict::shared::cta.cluster;`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadFenceSyncRestrict {
    /// `fence.acquire.sync_restrict::shared::cluster.cluster;`
    AcquireSharedCluster,
    /// `fence.release.sync_restrict::shared::cta.cluster;`
    ReleaseSharedCta,
}

/// `fence.op_restrict.release.scope;`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationFence {
    /// `.scope`
    pub scope: Scope,
}

/// `fence.proxy.proxykind;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyFence {
    /// `.proxykind`
    pub kind: ProxyKind,
}

/// `fence.proxy.tensormap::generic.release.scope;`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProxyTensormapRelease {
    /// `.scope`
    pub scope: Scope,
}

/// `fence.proxy.tensormap::generic.acquire.scope  [addr], size;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyTensormapAcquire {
    /// `.scope`
    pub scope: Scope,
    /// `[addr]`
    pub address: AddressOperand,
    /// `size`
    pub size: ProxySize,
}

/// `fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;`
/// `fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyAsync {
    /// `fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;`
    AcquireSharedCluster,
    /// `fence.proxy.async::generic.release.sync_restrict::shared::cta.cluster;`
    ReleaseSharedCta,
}

/// `membar.level;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldStyleScope {
    /// `.level`
    pub level: Level,
}

/// `membar.proxy.proxykind;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldStyleProxy {
    /// `.proxykind`
    pub kind: ProxyKind,
}

/// `.sem = { .sc, .acq_rel, .acquire, .release }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semantics {
    /// `.sc`
    Sc,
    /// `.acq_rel`
    AcqRel,
    /// `.acquire`
    Acquire,
    /// `.release`
    Release,
}

/// `.scope = { .cta, .cluster, .gpu, .sys }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// `.cta`
    Cta,
    /// `.cluster`
    Cluster,
    /// `.gpu`
    Gpu,
    /// `.sys`
    Sys,
}

/// `.proxykind = { .alias, .async, .async.global, .async.shared::{cta, cluster} }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyKind {
    /// `.alias`
    Alias,
    /// `.async`
    Async,
    /// `.async.global`
    AsyncGlobal,
    /// `.async.shared::cta`
    AsyncSharedCta,
    /// `.async.shared::cluster`
    AsyncSharedCluster,
}

/// `size = 128`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxySize {
    /// `size = 128`
    B128,
}

/// `.level = { .cta, .gl, .sys }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// `.cta`
    Cta,
    /// `.gl`
    Gl,
    /// `.sys`
    Sys,
}
