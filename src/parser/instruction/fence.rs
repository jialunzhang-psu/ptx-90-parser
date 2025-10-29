use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, Span, expect_directive_value,
        expect_identifier_value, peek_directive, unexpected_value,
    },
    r#type::{
        common::AddressOperand,
        instruction::fence::{
            FenceInstruction, FenceOperationRestrict, FenceOperationRestriction, FenceProxyAsync,
            FenceProxySharedScope, FenceProxySize, FenceProxyTensorMapAcquire,
            FenceProxyTensorMapRelease, FenceScope, FenceSemantics, FenceSyncRestrictShared,
            FenceSyncSemantics, FenceTensorMapDirection, FenceThread, FenceThreadSyncRestrict,
            MembarLevel, MembarProxy,
        },
    },
};

fn fence_semantics_from_directive(
    directive: &str,
    span: Span,
) -> Result<FenceSemantics, PtxParseError> {
    match directive {
        "sc" => Ok(FenceSemantics::Sc),
        "acq_rel" => Ok(FenceSemantics::AcqRel),
        "acquire" => Ok(FenceSemantics::Acquire),
        "release" => Ok(FenceSemantics::Release),
        other => Err(unexpected_value(
            span,
            &[".sc", ".acq_rel", ".acquire", ".release"],
            format!(".{other}"),
        )),
    }
}

fn expect_fence_semantics(
    stream: &mut PtxTokenStream,
) -> Result<(FenceSemantics, Span, String), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let semantics = fence_semantics_from_directive(&directive, span.clone())?;
    Ok((semantics, span, directive))
}

fn expect_sync_semantics(
    stream: &mut PtxTokenStream,
) -> Result<(FenceSyncSemantics, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let semantics = match directive.as_str() {
        "acquire" => FenceSyncSemantics::Acquire,
        "release" => FenceSyncSemantics::Release,
        other => {
            return Err(unexpected_value(
                span.clone(),
                &[".acquire", ".release"],
                format!(".{other}"),
            ));
        }
    };
    Ok((semantics, span))
}

fn fence_scope_from_directive(directive: &str, span: Span) -> Result<FenceScope, PtxParseError> {
    match directive {
        "cta" => Ok(FenceScope::Cta),
        "cluster" => Ok(FenceScope::Cluster),
        "gpu" => Ok(FenceScope::Gpu),
        "sys" => Ok(FenceScope::Sys),
        other => Err(unexpected_value(
            span,
            &[".cta", ".cluster", ".gpu", ".sys"],
            format!(".{other}"),
        )),
    }
}

fn expect_fence_scope(stream: &mut PtxTokenStream) -> Result<(FenceScope, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    Ok((fence_scope_from_directive(&directive, span.clone())?, span))
}

impl PtxParser for FenceSemantics {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(expect_fence_semantics(stream)?.0)
    }
}

impl PtxParser for FenceScope {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(expect_fence_scope(stream)?.0)
    }
}

impl PtxParser for FenceSyncSemantics {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(expect_sync_semantics(stream)?.0)
    }
}

impl PtxParser for FenceSyncRestrictShared {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_identifier()?;
        match value.as_str() {
            "cluster" => Ok(FenceSyncRestrictShared::Cluster),
            "cta" => Ok(FenceSyncRestrictShared::Cta),
            other => Err(unexpected_value(span, &["cluster", "cta"], other)),
        }
    }
}

impl PtxParser for FenceOperationRestriction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "mbarrier_init" => Ok(FenceOperationRestriction::MbarrierInit),
            other => Err(unexpected_value(
                span,
                &[".mbarrier_init"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for FenceTensorMapDirection {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_directive_value(stream, "tensormap")?;
        stream.expect_double_colon()?;
        let (identifier, span) = stream.expect_identifier()?;
        match identifier.as_str() {
            "generic" => Ok(FenceTensorMapDirection::TensormapFromGeneric),
            other => Err(unexpected_value(span, &["generic"], other)),
        }
    }
}

impl PtxParser for FenceProxySize {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (token, span) = stream.consume()?;
        match token {
            PtxToken::DecimalInteger(value) if value == "128" => Ok(FenceProxySize::N128),
            other => Err(unexpected_value(
                span.clone(),
                &["128"],
                format!("{other:?}"),
            )),
        }
    }
}

impl PtxParser for FenceProxySharedScope {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (identifier, span) = stream.expect_identifier()?;
        match identifier.as_str() {
            "cta" => Ok(FenceProxySharedScope::Cta),
            "cluster" => Ok(FenceProxySharedScope::Cluster),
            other => Err(unexpected_value(span, &["cta", "cluster"], other)),
        }
    }
}

impl PtxParser for MembarLevel {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "cta" => Ok(MembarLevel::Cta),
            "gl" => Ok(MembarLevel::Gl),
            "sys" => Ok(MembarLevel::Sys),
            other => Err(unexpected_value(
                span,
                &[".cta", ".gl", ".sys"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for MembarProxy {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "alias" => Ok(MembarProxy::Alias),
            "async" => {
                if stream
                    .check(|token| matches!(token, PtxToken::Directive(name) if name == "global"))
                {
                    stream.consume()?;
                    Ok(MembarProxy::AsyncGlobal)
                } else if stream
                    .check(|token| matches!(token, PtxToken::Directive(name) if name == "shared"))
                {
                    stream.consume()?;
                    stream.expect_double_colon()?;
                    let shared = FenceProxySharedScope::parse(stream)?;
                    Ok(MembarProxy::AsyncShared(shared))
                } else {
                    Ok(MembarProxy::Async)
                }
            }
            other => Err(unexpected_value(
                span,
                &[
                    ".alias",
                    ".async",
                    ".async.global",
                    ".async.shared::cta",
                    ".async.shared::cluster",
                ],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_thread_without_semantics(
    stream: &mut PtxTokenStream,
) -> Result<FenceInstruction, PtxParseError> {
    let scope = FenceScope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(FenceInstruction::Thread(FenceThread {
        semantics: None,
        scope,
    }))
}

fn parse_thread_with_semantics(
    stream: &mut PtxTokenStream,
    semantics: FenceSemantics,
) -> Result<FenceInstruction, PtxParseError> {
    let scope = FenceScope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(FenceInstruction::Thread(FenceThread {
        semantics: Some(semantics),
        scope,
    }))
}

fn parse_thread_sync_restrict(
    stream: &mut PtxTokenStream,
    semantics: FenceSyncSemantics,
) -> Result<FenceInstruction, PtxParseError> {
    expect_directive_value(stream, "sync_restrict")?;
    stream.expect_double_colon()?;
    expect_identifier_value(stream, "shared")?;
    stream.expect_double_colon()?;
    let shared = FenceSyncRestrictShared::parse(stream)?;
    let scope = FenceScope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(FenceInstruction::ThreadSyncRestrict(
        FenceThreadSyncRestrict {
            semantics,
            shared,
            scope,
        },
    ))
}

fn parse_operation_restrict(
    stream: &mut PtxTokenStream,
) -> Result<FenceInstruction, PtxParseError> {
    let restriction = FenceOperationRestriction::parse(stream)?;
    let semantics = FenceSyncSemantics::parse(stream)?;
    let scope = FenceScope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(FenceInstruction::OperationRestrict(
        FenceOperationRestrict {
            restriction,
            semantics,
            scope,
        },
    ))
}

fn parse_proxy_simple(stream: &mut PtxTokenStream) -> Result<FenceInstruction, PtxParseError> {
    let kind = MembarProxy::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(FenceInstruction::Proxy(kind))
}

fn parse_proxy_tensor_map(stream: &mut PtxTokenStream) -> Result<FenceInstruction, PtxParseError> {
    let direction = FenceTensorMapDirection::parse(stream)?;
    let semantics = FenceSyncSemantics::parse(stream)?;
    let (scope, _) = expect_fence_scope(stream)?;

    match semantics {
        FenceSyncSemantics::Release => {
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FenceInstruction::ProxyTensorMapRelease(
                FenceProxyTensorMapRelease { direction, scope },
            ))
        }
        FenceSyncSemantics::Acquire => {
            let address = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = FenceProxySize::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(FenceInstruction::ProxyTensorMapAcquire(
                FenceProxyTensorMapAcquire {
                    direction,
                    scope,
                    address,
                    size,
                },
            ))
        }
    }
}

fn parse_proxy_async(stream: &mut PtxTokenStream) -> Result<FenceInstruction, PtxParseError> {
    expect_directive_value(stream, "async")?;
    stream.expect_double_colon()?;
    let (identifier, span) = stream.expect_identifier()?;
    if identifier != "generic" {
        return Err(unexpected_value(span, &["generic"], identifier));
    }

    let semantics = FenceSyncSemantics::parse(stream)?;
    expect_directive_value(stream, "sync_restrict")?;
    stream.expect_double_colon()?;
    expect_identifier_value(stream, "shared")?;
    stream.expect_double_colon()?;
    let shared = FenceSyncRestrictShared::parse(stream)?;
    let scope = FenceScope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(FenceInstruction::ProxyAsync(FenceProxyAsync {
        semantics,
        shared,
        scope,
    }))
}

fn parse_proxy(stream: &mut PtxTokenStream) -> Result<FenceInstruction, PtxParseError> {
    expect_directive_value(stream, "proxy")?;

    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "tensormap")) {
        parse_proxy_tensor_map(stream)
    } else if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "async")) {
        let remaining = stream.remaining();
        if remaining.len() >= 2 && matches!(&remaining[1].0, PtxToken::Colon) {
            parse_proxy_async(stream)
        } else {
            parse_proxy_simple(stream)
        }
    } else {
        parse_proxy_simple(stream)
    }
}

fn parse_fence(stream: &mut PtxTokenStream, span: Span) -> Result<FenceInstruction, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "proxy")) {
        parse_proxy(stream)
    } else if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if name == "mbarrier_init"
        )
    }) {
        parse_operation_restrict(stream)
    } else if let Some((directive, directive_span)) = peek_directive(stream)? {
        match directive.as_str() {
            "sc" | "acq_rel" => {
                let (semantics, _, _) = expect_fence_semantics(stream)?;
                parse_thread_with_semantics(stream, semantics)
            }
            "acquire" | "release" => {
                let remaining = stream.remaining();
                if remaining.len() >= 2
                    && matches!(
                        &remaining[1].0,
                        PtxToken::Directive(name) if name == "sync_restrict"
                    )
                {
                    let (semantics, _) = expect_sync_semantics(stream)?;
                    parse_thread_sync_restrict(stream, semantics)
                } else {
                    let (semantics, _, _) = expect_fence_semantics(stream)?;
                    parse_thread_with_semantics(stream, semantics)
                }
            }
            "cta" | "cluster" | "gpu" | "sys" => parse_thread_without_semantics(stream),
            other => Err(unexpected_value(
                directive_span,
                &[
                    ".sc",
                    ".acq_rel",
                    ".acquire",
                    ".release",
                    ".cta",
                    ".cluster",
                    ".gpu",
                    ".sys",
                    ".proxy",
                    ".mbarrier_init",
                ],
                format!(".{other}"),
            )),
        }
    } else {
        Err(unexpected_value(
            span,
            &[
                "fence{.sem}.scope",
                "fence.acquire.sync_restrict::shared::*",
                "fence.release.sync_restrict::shared::*",
                "fence.mbarrier_init.release.scope",
                "fence.proxy.*",
            ],
            "fence",
        ))
    }
}

fn parse_membar(stream: &mut PtxTokenStream) -> Result<FenceInstruction, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "proxy")) {
        expect_directive_value(stream, "proxy")?;
        let proxy = MembarProxy::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(FenceInstruction::MembarProxy(proxy))
    } else {
        let level = MembarLevel::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(FenceInstruction::MembarScope(level))
    }
}

impl PtxParser for FenceInstruction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "fence" => parse_fence(stream, span),
            "membar" => parse_membar(stream),
            other => Err(unexpected_value(span, &["fence", "membar"], other)),
        }
    }
}
