use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::membar::{
            Membar, OldStyleProxy, ProxyKind, ProxySize, ProxyTensormapAcquire, Scope, Semantics,
            ThreadFence, ThreadFenceSyncRestrict,
        },
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Membar>(source);
}

#[test]
fn parses_simple_thread_fence_with_semantics() {
    let source = "fence.sc.cta;";
    assert_eq!(
        parse::<Membar>(source),
        Membar::ThreadFence(ThreadFence {
            semantics: Some(Semantics::Sc),
            scope: Scope::Cta,
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_thread_fence_sync_restrict_variant() {
    let source = "fence.acquire.sync_restrict::shared::cluster.cluster;";
    assert_eq!(
        parse::<Membar>(source),
        Membar::ThreadFenceSyncRestrict(ThreadFenceSyncRestrict::AcquireSharedCluster)
    );
    assert_roundtrip(source);
}

#[test]
fn parses_proxy_tensormap_acquire_variant() {
    let source = "fence.proxy.tensormap::generic.acquire.cluster[%rd1], size = 128;";
    assert_eq!(
        parse::<Membar>(source),
        Membar::ProxyTensormapAcquire(ProxyTensormapAcquire {
            scope: Scope::Cluster,
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None
            ),
            size: ProxySize::B128,
        })
    );
    assert_roundtrip(source);
}

#[test]
fn parses_old_style_proxy_variant() {
    let source = "membar.proxy.async.shared::cta;";
    assert_eq!(
        parse::<Membar>(source),
        Membar::OldStyleProxy(OldStyleProxy {
            kind: ProxyKind::AsyncSharedCta
        })
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_thread_fence_sync_restrict_with_invalid_semantics() {
    let err = parse_result::<Membar>("fence.sc.sync_restrict::shared::cluster.cluster;")
        .expect_err("parse should fail when semantics are invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("fence.acquire.sync_restrict::shared::cluster.cluster;");
}

#[test]
fn rejects_proxy_tensormap_with_invalid_size() {
    let err =
        parse_result::<Membar>("fence.proxy.tensormap::generic.acquire.cluster[%rd1], size = 64;")
            .expect_err("parse should fail on invalid size literal");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::InvalidLiteral(_)
    ));
    assert_roundtrip("fence.proxy.tensormap::generic.acquire.cluster[%rd1], size = 128;");
}
