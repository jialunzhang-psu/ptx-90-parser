use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, VariableSymbol},
        instruction::fence::{
            FenceInstruction, FenceOperationRestrict, FenceOperationRestriction, FenceProxyAsync,
            FenceProxySharedScope, FenceProxySize, FenceProxyTensorMapAcquire,
            FenceProxyTensorMapRelease, FenceScope, FenceSemantics, FenceSyncRestrictShared,
            FenceSyncSemantics, FenceTensorMapDirection, FenceThread, FenceThreadSyncRestrict,
            MembarLevel, MembarProxy,
        },
    },
};

#[test]
fn parses_thread_fence_with_semantics() {
    assert_eq!(
        parse::<FenceInstruction>("fence.sc.cta;"),
        FenceInstruction::Thread(FenceThread {
            semantics: Some(FenceSemantics::Sc),
            scope: FenceScope::Cta,
        })
    );
    assert_roundtrip::<FenceInstruction>("fence.sc.cta;");
}

#[test]
fn parses_thread_sync_restrict_variant() {
    assert_eq!(
        parse::<FenceInstruction>("fence.acquire.sync_restrict::shared::cluster.cluster;"),
        FenceInstruction::ThreadSyncRestrict(FenceThreadSyncRestrict {
            semantics: FenceSyncSemantics::Acquire,
            shared: FenceSyncRestrictShared::Cluster,
            scope: FenceScope::Cluster,
        })
    );
    assert_roundtrip::<FenceInstruction>("fence.acquire.sync_restrict::shared::cluster.cluster;");
}

#[test]
fn parses_operation_restriction_variant() {
    assert_eq!(
        parse::<FenceInstruction>("fence.mbarrier_init.release.cluster;"),
        FenceInstruction::OperationRestrict(FenceOperationRestrict {
            restriction: FenceOperationRestriction::MbarrierInit,
            semantics: FenceSyncSemantics::Release,
            scope: FenceScope::Cluster,
        })
    );
    assert_roundtrip::<FenceInstruction>("fence.mbarrier_init.release.cluster;");
}

#[test]
fn parses_proxy_tensormap_release_variant() {
    assert_eq!(
        parse::<FenceInstruction>("fence.proxy.tensormap::generic.release.gpu;"),
        FenceInstruction::ProxyTensorMapRelease(FenceProxyTensorMapRelease {
            direction: FenceTensorMapDirection::TensormapFromGeneric,
            scope: FenceScope::Gpu,
        })
    );
    assert_roundtrip::<FenceInstruction>("fence.proxy.tensormap::generic.release.gpu;");
}

#[test]
fn parses_proxy_tensormap_acquire_variant() {
    assert_eq!(
        parse::<FenceInstruction>("fence.proxy.tensormap::generic.acquire.gpu [tmap], 128;"),
        FenceInstruction::ProxyTensorMapAcquire(FenceProxyTensorMapAcquire {
            direction: FenceTensorMapDirection::TensormapFromGeneric,
            scope: FenceScope::Gpu,
            address: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("tmap".into())),
                None
            ),
            size: FenceProxySize::N128,
        })
    );
    assert_roundtrip::<FenceInstruction>("fence.proxy.tensormap::generic.acquire.gpu [tmap], 128;");
}

#[test]
fn parses_proxy_async_variant() {
    assert_eq!(
        parse::<FenceInstruction>(
            "fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;"
        ),
        FenceInstruction::ProxyAsync(FenceProxyAsync {
            semantics: FenceSyncSemantics::Acquire,
            shared: FenceSyncRestrictShared::Cluster,
            scope: FenceScope::Cluster,
        })
    );
    assert_roundtrip::<FenceInstruction>(
        "fence.proxy.async::generic.acquire.sync_restrict::shared::cluster.cluster;",
    );
}

#[test]
fn parses_membar_variants() {
    assert_eq!(
        parse::<FenceInstruction>("membar.gl;"),
        FenceInstruction::MembarScope(MembarLevel::Gl)
    );
    assert_roundtrip::<FenceInstruction>("membar.gl;");
    assert_eq!(
        parse::<FenceInstruction>("membar.proxy.async.shared::cta;"),
        FenceInstruction::MembarProxy(MembarProxy::AsyncShared(FenceProxySharedScope::Cta))
    );
    assert_roundtrip::<FenceInstruction>("membar.proxy.async.shared::cta;");
}

#[test]
fn rejects_thread_sync_restrict_with_invalid_semantics() {
    let err = parse_result::<FenceInstruction>("fence.sc.sync_restrict::shared::cluster.cluster;")
        .expect_err("parser should reject sync_restrict when semantics are not acquire/release");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_proxy_tensormap_with_invalid_size() {
    let err =
        parse_result::<FenceInstruction>("fence.proxy.tensormap::generic.acquire.gpu [tmap], 64;")
            .expect_err("parser should reject proxy tensormap acquire with unsupported size");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::InvalidLiteral(_)
    ));
}

#[test]
fn rejects_membar_proxy_with_unknown_kind() {
    let err = parse_result::<FenceInstruction>("membar.proxy.foo;")
        .expect_err("parser should reject unknown membar proxy kind");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
