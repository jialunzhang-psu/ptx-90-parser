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

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::membar::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Level {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta"), |_, _span| Level::Cta),
                map(string_p(".sys"), |_, _span| Level::Sys),
                map(string_p(".gl"), |_, _span| Level::Gl)
            )
        }
    }

    impl PtxParser for OpRestrict {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".mbarrier_init"), |_, _span| {
                OpRestrict::MbarrierInit
            }))
        }
    }

    impl PtxParser for Proxykind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".async.shared::cluster"), |_, _span| {
                    Proxykind::AsyncSharedCluster
                }),
                map(string_p(".async.shared::cta"), |_, _span| {
                    Proxykind::AsyncSharedCta
                }),
                map(string_p(".async.global"), |_, _span| Proxykind::AsyncGlobal),
                map(string_p(".alias"), |_, _span| Proxykind::Alias),
                map(string_p(".async"), |_, _span| Proxykind::Async)
            )
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".cta"), |_, _span| Scope::Cta),
                map(string_p(".gpu"), |_, _span| Scope::Gpu),
                map(string_p(".sys"), |_, _span| Scope::Sys)
            )
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".acq_rel"), |_, _span| Sem::AcqRel),
                map(string_p(".acquire"), |_, _span| Sem::Acquire),
                map(string_p(".release"), |_, _span| Sem::Release),
                map(string_p(".sc"), |_, _span| Sem::Sc)
            )
        }
    }

    impl PtxParser for ToProxykindFromProxykind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tensormap::generic"), |_, _span| {
                ToProxykindFromProxykind::TensormapGeneric
            }))
        }
    }

    impl PtxParser for FenceSemScope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    optional(Sem::parse()),
                    Scope::parse(),
                    semicolon_p()
                ),
                |(_, sem, scope, _), span| {
                    ok!(FenceSemScope {
                        sem = sem,
                        scope = scope,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceAcquireSyncRestrictSharedClusterCluster {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".acquire"),
                    string_p(".sync_restrict::shared::cluster"),
                    string_p(".cluster"),
                    semicolon_p()
                ),
                |(_, acquire, sync_restrict_shared_cluster, cluster, _), span| {
                    ok!(FenceAcquireSyncRestrictSharedClusterCluster {
                        acquire = acquire,
                        sync_restrict_shared_cluster = sync_restrict_shared_cluster,
                        cluster = cluster,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceReleaseSyncRestrictSharedCtaCluster {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".release"),
                    string_p(".sync_restrict::shared::cta"),
                    string_p(".cluster"),
                    semicolon_p()
                ),
                |(_, release, sync_restrict_shared_cta, cluster, _), span| {
                    ok!(FenceReleaseSyncRestrictSharedCtaCluster {
                        release = release,
                        sync_restrict_shared_cta = sync_restrict_shared_cta,
                        cluster = cluster,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceOpRestrictReleaseCluster {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    OpRestrict::parse(),
                    string_p(".release"),
                    string_p(".cluster"),
                    semicolon_p()
                ),
                |(_, op_restrict, release, cluster, _), span| {
                    ok!(FenceOpRestrictReleaseCluster {
                        op_restrict = op_restrict,
                        release = release,
                        cluster = cluster,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceProxyProxykind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".proxy"),
                    Proxykind::parse(),
                    semicolon_p()
                ),
                |(_, proxy, proxykind, _), span| {
                    ok!(FenceProxyProxykind {
                        proxy = proxy,
                        proxykind = proxykind,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceProxyToProxykindFromProxykindReleaseScope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".proxy"),
                    ToProxykindFromProxykind::parse(),
                    string_p(".release"),
                    Scope::parse(),
                    semicolon_p()
                ),
                |(_, proxy, to_proxykind_from_proxykind, release, scope, _), span| {
                    ok!(FenceProxyToProxykindFromProxykindReleaseScope {
                        proxy = proxy,
                        to_proxykind_from_proxykind = to_proxykind_from_proxykind,
                        release = release,
                        scope = scope,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceProxyToProxykindFromProxykindAcquireScope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".proxy"),
                    ToProxykindFromProxykind::parse(),
                    string_p(".acquire"),
                    Scope::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, proxy, to_proxykind_from_proxykind, acquire, scope, addr, _, size, _),
                 span| {
                    ok!(FenceProxyToProxykindFromProxykindAcquireScope {
                        proxy = proxy,
                        to_proxykind_from_proxykind = to_proxykind_from_proxykind,
                        acquire = acquire,
                        scope = scope,
                        addr = addr,
                        size = size,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".proxy"),
                    string_p(".async::generic"),
                    string_p(".acquire"),
                    string_p(".sync_restrict::shared::cluster"),
                    string_p(".cluster"),
                    semicolon_p()
                ),
                |(_, proxy, async_generic, acquire, sync_restrict_shared_cluster, cluster, _),
                 span| {
                    ok!(FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
                        proxy = proxy,
                        async_generic = async_generic,
                        acquire = acquire,
                        sync_restrict_shared_cluster = sync_restrict_shared_cluster,
                        cluster = cluster,

                    })
                },
            )
        }
    }

    impl PtxParser for FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fence"),
                    string_p(".proxy"),
                    string_p(".async::generic"),
                    string_p(".release"),
                    string_p(".sync_restrict::shared::cta"),
                    string_p(".cluster"),
                    semicolon_p()
                ),
                |(_, proxy, async_generic, release, sync_restrict_shared_cta, cluster, _), span| {
                    ok!(FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
                        proxy = proxy,
                        async_generic = async_generic,
                        release = release,
                        sync_restrict_shared_cta = sync_restrict_shared_cta,
                        cluster = cluster,

                    })
                },
            )
        }
    }

    impl PtxParser for MembarLevel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(string_p("membar"), Level::parse(), semicolon_p()),
                |(_, level, _), span| {
                    ok!(MembarLevel {
                        level = level,

                    })
                },
            )
        }
    }

    impl PtxParser for MembarProxyProxykind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("membar"),
                    string_p(".proxy"),
                    Proxykind::parse(),
                    semicolon_p()
                ),
                |(_, proxy, proxykind, _), span| {
                    ok!(MembarProxyProxykind {
                        proxy = proxy,
                        proxykind = proxykind,

                    })
                },
            )
        }
    }
}
