use crate::{lexer::PtxToken, r#type::instruction::membar::*, unparser::*};

impl PtxUnparser for Semantics {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Semantics::Sc => "sc",
            Semantics::AcqRel => "acq_rel",
            Semantics::Acquire => "acquire",
            Semantics::Release => "release",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for Scope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Scope::Cta => "cta",
            Scope::Cluster => "cluster",
            Scope::Gpu => "gpu",
            Scope::Sys => "sys",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for ProxyKind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ProxyKind::Alias => push_directive(tokens, "alias"),
            ProxyKind::Async => push_directive(tokens, "async"),
            ProxyKind::AsyncGlobal => {
                push_directive(tokens, "async");
                push_directive(tokens, "global");
            }
            ProxyKind::AsyncSharedCta => {
                push_directive(tokens, "async");
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
            ProxyKind::AsyncSharedCluster => {
                push_directive(tokens, "async");
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cluster");
            }
        }
    }
}

impl PtxUnparser for ProxySize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ProxySize::B128 => {
                push_identifier(tokens, "size");
                tokens.push(PtxToken::Equals);
                tokens.push(PtxToken::DecimalInteger("128".to_string()));
            }
        }
    }
}

impl PtxUnparser for Level {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Level::Cta => "cta",
            Level::Gl => "gl",
            Level::Sys => "sys",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for ThreadFence {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        if let Some(semantics) = &self.semantics {
            semantics.unparse_tokens(tokens);
        }
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ThreadFenceSyncRestrict {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        let (semantics, level) = match self {
            ThreadFenceSyncRestrict::AcquireSharedCluster => (Semantics::Acquire, "cluster"),
            ThreadFenceSyncRestrict::ReleaseSharedCta => (Semantics::Release, "cta"),
        };
        semantics.unparse_tokens(tokens);
        push_directive(tokens, "sync_restrict");
        push_double_colon(tokens);
        push_identifier(tokens, "shared");
        push_double_colon(tokens);
        push_identifier(tokens, level);
        Scope::Cluster.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for OperationFence {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "op_restrict");
        Semantics::Release.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ProxyFence {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        self.kind.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ProxyTensormapRelease {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        push_directive(tokens, "tensormap");
        push_double_colon(tokens);
        push_identifier(tokens, "generic");
        Semantics::Release.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ProxyTensormapAcquire {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        push_directive(tokens, "tensormap");
        push_double_colon(tokens);
        push_identifier(tokens, "generic");
        Semantics::Acquire.unparse_tokens(tokens);
        self.scope.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.size.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ProxyAsync {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "fence");
        push_directive(tokens, "proxy");
        push_directive(tokens, "async");
        push_double_colon(tokens);
        push_identifier(tokens, "generic");
        let (semantics, level) = match self {
            ProxyAsync::AcquireSharedCluster => (Semantics::Acquire, "cluster"),
            ProxyAsync::ReleaseSharedCta => (Semantics::Release, "cta"),
        };
        semantics.unparse_tokens(tokens);
        push_directive(tokens, "sync_restrict");
        push_double_colon(tokens);
        push_identifier(tokens, "shared");
        push_double_colon(tokens);
        push_identifier(tokens, level);
        Scope::Cluster.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for OldStyleScope {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "membar");
        self.level.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for OldStyleProxy {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "membar");
        push_directive(tokens, "proxy");
        self.kind.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Membar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Membar::ThreadFence(value) => value.unparse_tokens(tokens),
            Membar::ThreadFenceSyncRestrict(value) => value.unparse_tokens(tokens),
            Membar::OperationFence(value) => value.unparse_tokens(tokens),
            Membar::ProxyFence(value) => value.unparse_tokens(tokens),
            Membar::ProxyTensormapRelease(value) => value.unparse_tokens(tokens),
            Membar::ProxyTensormapAcquire(value) => value.unparse_tokens(tokens),
            Membar::ProxyAsync(value) => value.unparse_tokens(tokens),
            Membar::OldStyleScope(value) => value.unparse_tokens(tokens),
            Membar::OldStyleProxy(value) => value.unparse_tokens(tokens),
        }
    }
}
