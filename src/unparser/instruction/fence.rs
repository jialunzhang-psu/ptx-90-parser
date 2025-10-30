use crate::{
    lexer::PtxToken,
    r#type::{common::AddressOperand, instruction::fence::*},
    unparser::*,
};

fn push_address(tokens: &mut Vec<PtxToken>, address: &AddressOperand) {
    address.unparse_tokens(tokens);
}

impl PtxUnparser for FenceSemantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceSemantics::Sc => "sc",
            FenceSemantics::AcqRel => "acq_rel",
            FenceSemantics::Acquire => "acquire",
            FenceSemantics::Release => "release",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FenceScope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceScope::Cta => "cta",
            FenceScope::Cluster => "cluster",
            FenceScope::Gpu => "gpu",
            FenceScope::Sys => "sys",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FenceSyncSemantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceSyncSemantics::Acquire => "acquire",
            FenceSyncSemantics::Release => "release",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FenceSyncRestrictShared {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceSyncRestrictShared::Cluster => "cluster",
            FenceSyncRestrictShared::Cta => "cta",
        };
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for FenceOperationRestriction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceOperationRestriction::MbarrierInit => "mbarrier_init",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for FenceTensorMapDirection {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "tensormap");
        push_double_colon(tokens);
        push_identifier(tokens, "generic");
    }
}

impl PtxUnparser for FenceProxySize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FenceProxySize::N128 => tokens.push(PtxToken::DecimalInteger("128".to_string())),
        }
    }
}

impl PtxUnparser for FenceProxySharedScope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            FenceProxySharedScope::Cta => "cta",
            FenceProxySharedScope::Cluster => "cluster",
        };
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for FenceThread {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for FenceThreadSyncRestrict {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        self.semantics.unparse_tokens(tokens);
        push_directive(tokens, "sync_restrict");
        push_double_colon(tokens);
        push_identifier(tokens, "shared");
        push_double_colon(tokens);
        self.shared.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for FenceOperationRestrict {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        self.restriction.unparse_tokens(tokens);
        self.semantics.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for FenceProxyTensorMapRelease {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        self.direction.unparse_tokens(tokens);
        FenceSyncSemantics::Release.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for FenceProxyTensorMapAcquire {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        self.direction.unparse_tokens(tokens);
        FenceSyncSemantics::Acquire.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        push_address(tokens, &self.address);
        tokens.push(PtxToken::Comma);
        self.size.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for FenceProxyAsync {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        push_directive(tokens, "async");
        push_double_colon(tokens);
        push_identifier(tokens, "generic");
        self.semantics.unparse_tokens(tokens);
        push_directive(tokens, "sync_restrict");
        push_double_colon(tokens);
        push_identifier(tokens, "shared");
        push_double_colon(tokens);
        self.shared.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for MembarLevel {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            MembarLevel::Cta => "cta",
            MembarLevel::Gl => "gl",
            MembarLevel::Sys => "sys",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for MembarProxy {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            MembarProxy::Alias => push_directive(tokens, "alias"),
            MembarProxy::Async => push_directive(tokens, "async"),
            MembarProxy::AsyncGlobal => {
                push_directive(tokens, "async");
                push_directive(tokens, "global");
            }
            MembarProxy::AsyncShared(scope) => {
                push_directive(tokens, "async");
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                scope.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for FenceInstruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FenceInstruction::Thread(thread) => thread.unparse_tokens(tokens),
            FenceInstruction::ThreadSyncRestrict(sync) => sync.unparse_tokens(tokens),
            FenceInstruction::OperationRestrict(op) => op.unparse_tokens(tokens),
            FenceInstruction::Proxy(proxy) => {
                push_identifier(tokens, "fence");
                push_directive(tokens, "proxy");
                proxy.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            FenceInstruction::ProxyTensorMapRelease(release) => {
                release.unparse_tokens(tokens);
            }
            FenceInstruction::ProxyTensorMapAcquire(acquire) => {
                acquire.unparse_tokens(tokens);
            }
            FenceInstruction::ProxyAsync(async_proxy) => async_proxy.unparse_tokens(tokens),
            FenceInstruction::MembarScope(level) => {
                push_identifier(tokens, "membar");
                level.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            FenceInstruction::MembarProxy(proxy) => {
                push_identifier(tokens, "membar");
                push_directive(tokens, "proxy");
                proxy.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
