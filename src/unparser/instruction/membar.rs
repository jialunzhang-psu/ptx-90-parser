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
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::membar::section_0::*;

    impl PtxUnparser for FenceSemScope {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            if let Some(sem_0) = self.sem.as_ref() {
                match sem_0 {
                    Sem::AcqRel => {
                        push_directive(tokens, "acq_rel");
                    }
                    Sem::Acquire => {
                        push_directive(tokens, "acquire");
                    }
                    Sem::Release => {
                        push_directive(tokens, "release");
                    }
                    Sem::Sc => {
                        push_directive(tokens, "sc");
                    }
                }
            }
            match &self.scope {
                Scope::Cluster => {
                    push_directive(tokens, "cluster");
                }
                Scope::Cta => {
                    push_directive(tokens, "cta");
                }
                Scope::Gpu => {
                    push_directive(tokens, "gpu");
                }
                Scope::Sys => {
                    push_directive(tokens, "sys");
                }
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceAcquireSyncRestrictSharedClusterCluster {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "acquire");
            push_directive(tokens, "sync_restrict::shared::cluster");
            push_directive(tokens, "cluster");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceReleaseSyncRestrictSharedCtaCluster {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "release");
            push_directive(tokens, "sync_restrict::shared::cta");
            push_directive(tokens, "cluster");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceOpRestrictReleaseCluster {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            match &self.op_restrict {
                OpRestrict::MbarrierInit => {
                    push_directive(tokens, "mbarrier_init");
                }
            }
            push_directive(tokens, "release");
            push_directive(tokens, "cluster");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceProxyProxykind {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "proxy");
            match &self.proxykind {
                Proxykind::AsyncSharedCluster => {
                    push_directive(tokens, "async.shared::cluster");
                }
                Proxykind::AsyncSharedCta => {
                    push_directive(tokens, "async.shared::cta");
                }
                Proxykind::AsyncGlobal => {
                    push_directive(tokens, "async.global");
                }
                Proxykind::Alias => {
                    push_directive(tokens, "alias");
                }
                Proxykind::Async => {
                    push_directive(tokens, "async");
                }
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceProxyToProxykindFromProxykindReleaseScope {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "proxy");
            match &self.to_proxykind_from_proxykind {
                ToProxykindFromProxykind::TensormapGeneric => {
                    push_directive(tokens, "tensormap::generic");
                }
            }
            push_directive(tokens, "release");
            match &self.scope {
                Scope::Cluster => {
                    push_directive(tokens, "cluster");
                }
                Scope::Cta => {
                    push_directive(tokens, "cta");
                }
                Scope::Gpu => {
                    push_directive(tokens, "gpu");
                }
                Scope::Sys => {
                    push_directive(tokens, "sys");
                }
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceProxyToProxykindFromProxykindAcquireScope {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "proxy");
            match &self.to_proxykind_from_proxykind {
                ToProxykindFromProxykind::TensormapGeneric => {
                    push_directive(tokens, "tensormap::generic");
                }
            }
            push_directive(tokens, "acquire");
            match &self.scope {
                Scope::Cluster => {
                    push_directive(tokens, "cluster");
                }
                Scope::Cta => {
                    push_directive(tokens, "cta");
                }
                Scope::Gpu => {
                    push_directive(tokens, "gpu");
                }
                Scope::Sys => {
                    push_directive(tokens, "sys");
                }
            }
            self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.size.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "proxy");
            push_directive(tokens, "async::generic");
            push_directive(tokens, "acquire");
            push_directive(tokens, "sync_restrict::shared::cluster");
            push_directive(tokens, "cluster");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "fence");
            push_directive(tokens, "proxy");
            push_directive(tokens, "async::generic");
            push_directive(tokens, "release");
            push_directive(tokens, "sync_restrict::shared::cta");
            push_directive(tokens, "cluster");
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MembarLevel {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "membar");
            match &self.level {
                Level::Cta => {
                    push_directive(tokens, "cta");
                }
                Level::Sys => {
                    push_directive(tokens, "sys");
                }
                Level::Gl => {
                    push_directive(tokens, "gl");
                }
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for MembarProxyProxykind {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "membar");
            push_directive(tokens, "proxy");
            match &self.proxykind {
                Proxykind::AsyncSharedCluster => {
                    push_directive(tokens, "async.shared::cluster");
                }
                Proxykind::AsyncSharedCta => {
                    push_directive(tokens, "async.shared::cta");
                }
                Proxykind::AsyncGlobal => {
                    push_directive(tokens, "async.global");
                }
                Proxykind::Alias => {
                    push_directive(tokens, "alias");
                }
                Proxykind::Async => {
                    push_directive(tokens, "async");
                }
            }
            tokens.push(PtxToken::Semicolon);
        }
    }
}
