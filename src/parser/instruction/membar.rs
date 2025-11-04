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

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::membar::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for OpRestrict {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierInit
            {
                let saved_pos = stream.position();
                if stream.expect_string(".mbarrier_init").is_ok() {
                    return Ok(OpRestrict::MbarrierInit);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier_init"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Proxykind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Alias
            {
                let saved_pos = stream.position();
                if stream.expect_string(".alias").is_ok() {
                    return Ok(Proxykind::Alias);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Async
            {
                let saved_pos = stream.position();
                if stream.expect_string(".async").is_ok() {
                    return Ok(Proxykind::Async);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AsyncGlobal
            {
                let saved_pos = stream.position();
                if stream.expect_string(".async.global").is_ok() {
                    return Ok(Proxykind::AsyncGlobal);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AsyncSharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".async.shared::cta").is_ok() {
                    return Ok(Proxykind::AsyncSharedCta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AsyncSharedCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".async.shared::cluster").is_ok() {
                    return Ok(Proxykind::AsyncSharedCluster);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".alias", ".async", ".async.global", ".async.shared::cta", ".async.shared::cluster"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ToProxykindFromProxykind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try TensormapGeneric
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tensormap::generic").is_ok() {
                    return Ok(ToProxykindFromProxykind::TensormapGeneric);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tensormap::generic"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Sem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Sc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sc").is_ok() {
                    return Ok(Sem::Sc);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try AcqRel
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acq_rel").is_ok() {
                    return Ok(Sem::AcqRel);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Acquire
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acquire").is_ok() {
                    return Ok(Sem::Acquire);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Sem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".sc", ".acq_rel", ".acquire", ".release"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta", ".cluster", ".gpu", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Level {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Level::Cta);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Gl
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gl").is_ok() {
                    return Ok(Level::Gl);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Level::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta", ".gl", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for FenceSemScope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            let scope = Scope::parse(stream)?;
            Ok(FenceSemScope {
                sem,
                scope,
            })
        }
    }


    impl PtxParser for FenceAcquireSyncRestrictSharedClusterCluster {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".acquire")?;
            let acquire = ();
            stream.expect_string(".sync_restrict::shared::cluster")?;
            let sync_restrict_shared_cluster = ();
            stream.expect_string(".cluster")?;
            let cluster = ();
            Ok(FenceAcquireSyncRestrictSharedClusterCluster {
                acquire,
                sync_restrict_shared_cluster,
                cluster,
            })
        }
    }


    impl PtxParser for FenceReleaseSyncRestrictSharedCtaCluster {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".release")?;
            let release = ();
            stream.expect_string(".sync_restrict::shared::cta")?;
            let sync_restrict_shared_cta = ();
            stream.expect_string(".cluster")?;
            let cluster = ();
            Ok(FenceReleaseSyncRestrictSharedCtaCluster {
                release,
                sync_restrict_shared_cta,
                cluster,
            })
        }
    }


    impl PtxParser for FenceOpRestrictReleaseCluster {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            let op_restrict = OpRestrict::parse(stream)?;
            stream.expect_string(".release")?;
            let release = ();
            stream.expect_string(".cluster")?;
            let cluster = ();
            Ok(FenceOpRestrictReleaseCluster {
                op_restrict,
                release,
                cluster,
            })
        }
    }


    impl PtxParser for FenceProxyProxykind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            let proxykind = Proxykind::parse(stream)?;
            Ok(FenceProxyProxykind {
                proxy,
                proxykind,
            })
        }
    }


    impl PtxParser for FenceProxyToProxykindFromProxykindReleaseScope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            let to_proxykind_from_proxykind = ToProxykindFromProxykind::parse(stream)?;
            stream.expect_string(".release")?;
            let release = ();
            let scope = Scope::parse(stream)?;
            Ok(FenceProxyToProxykindFromProxykindReleaseScope {
                proxy,
                to_proxykind_from_proxykind,
                release,
                scope,
            })
        }
    }


    impl PtxParser for FenceProxyToProxykindFromProxykindAcquireScope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            let to_proxykind_from_proxykind = ToProxykindFromProxykind::parse(stream)?;
            stream.expect_string(".acquire")?;
            let acquire = ();
            let scope = Scope::parse(stream)?;
            let addr = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            Ok(FenceProxyToProxykindFromProxykindAcquireScope {
                proxy,
                to_proxykind_from_proxykind,
                acquire,
                scope,
                addr,
                size,
            })
        }
    }


    impl PtxParser for FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            stream.expect_string(".async::generic")?;
            let async_generic = ();
            stream.expect_string(".acquire")?;
            let acquire = ();
            stream.expect_string(".sync_restrict::shared::cluster")?;
            let sync_restrict_shared_cluster = ();
            stream.expect_string(".cluster")?;
            let cluster = ();
            Ok(FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
                proxy,
                async_generic,
                acquire,
                sync_restrict_shared_cluster,
                cluster,
            })
        }
    }


    impl PtxParser for FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("fence")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            stream.expect_string(".async::generic")?;
            let async_generic = ();
            stream.expect_string(".release")?;
            let release = ();
            stream.expect_string(".sync_restrict::shared::cta")?;
            let sync_restrict_shared_cta = ();
            stream.expect_string(".cluster")?;
            let cluster = ();
            Ok(FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
                proxy,
                async_generic,
                release,
                sync_restrict_shared_cta,
                cluster,
            })
        }
    }


    impl PtxParser for MembarLevel {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("membar")?;
            let level = Level::parse(stream)?;
            Ok(MembarLevel {
                level,
            })
        }
    }


    impl PtxParser for MembarProxyProxykind {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("membar")?;
            stream.expect_string(".proxy")?;
            let proxy = ();
            let proxykind = Proxykind::parse(stream)?;
            Ok(MembarProxyProxykind {
                proxy,
                proxykind,
            })
        }
    }


}

