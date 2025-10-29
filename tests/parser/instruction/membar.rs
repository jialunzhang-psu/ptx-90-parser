use crate::util::{parse, parse_result};
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

#[test]
fn parses_simple_thread_fence_with_semantics() {
    assert_eq!(
        parse::<Membar>("fence.sc.cta;"),
        Membar::ThreadFence(ThreadFence {
            semantics: Some(Semantics::Sc),
            scope: Scope::Cta,
        })
    );
}

#[test]
fn parses_thread_fence_sync_restrict_variant() {
    assert_eq!(
        parse::<Membar>("fence.acquire.sync_restrict::shared::cluster.cluster;"),
        Membar::ThreadFenceSyncRestrict(ThreadFenceSyncRestrict::AcquireSharedCluster)
    );
}

#[test]
fn parses_proxy_tensormap_acquire_variant() {
    assert_eq!(
        parse::<Membar>("fence.proxy.tensormap::generic.acquire.cluster[%rd1], size = 128;"),
        Membar::ProxyTensormapAcquire(ProxyTensormapAcquire {
            scope: Scope::Cluster,
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None
            ),
            size: ProxySize::B128,
        })
    );
}

#[test]
fn parses_old_style_proxy_variant() {
    assert_eq!(
        parse::<Membar>("membar.proxy.async.shared::cta;"),
        Membar::OldStyleProxy(OldStyleProxy {
            kind: ProxyKind::AsyncSharedCta
        })
    );
}

#[test]
fn rejects_thread_fence_sync_restrict_with_invalid_semantics() {
    let err = parse_result::<Membar>("fence.sc.sync_restrict::shared::cluster.cluster;")
        .expect_err("parse should fail when semantics are invalid");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
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
}
