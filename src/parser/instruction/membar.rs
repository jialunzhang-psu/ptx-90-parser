use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span, unexpected_value},
    r#type::{
        common::AddressOperand,
        instruction::membar::{
            Level, Membar, OldStyleProxy, OldStyleScope, OperationFence, ProxyAsync, ProxyFence,
            ProxyKind, ProxySize, ProxyTensormapAcquire, ProxyTensormapRelease, Scope, Semantics,
            ThreadFence, ThreadFenceSyncRestrict,
        },
    },
};

fn is_semantics_name(name: &str) -> bool {
    matches!(name, "sc" | "acq_rel" | "acquire" | "release")
}

fn is_scope_name(name: &str) -> bool {
    matches!(name, "cta" | "cluster" | "gpu" | "sys")
}

fn semantics_from_directive(directive: &str, span: Span) -> Result<Semantics, PtxParseError> {
    match directive {
        "sc" => Ok(Semantics::Sc),
        "acq_rel" => Ok(Semantics::AcqRel),
        "acquire" => Ok(Semantics::Acquire),
        "release" => Ok(Semantics::Release),
        other => Err(unexpected_value(
            span,
            &[".sc", ".acq_rel", ".acquire", ".release"],
            format!(".{other}"),
        )),
    }
}

fn expect_semantics(
    stream: &mut PtxTokenStream,
) -> Result<(Semantics, Span, String), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let semantics = semantics_from_directive(&directive, span.clone())?;
    Ok((semantics, span, directive))
}

impl PtxParser for Semantics {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(expect_semantics(stream)?.0)
    }
}

fn scope_from_directive(directive: &str, span: Span) -> Result<Scope, PtxParseError> {
    match directive {
        "cta" => Ok(Scope::Cta),
        "cluster" => Ok(Scope::Cluster),
        "gpu" => Ok(Scope::Gpu),
        "sys" => Ok(Scope::Sys),
        other => Err(unexpected_value(
            span,
            &[".cta", ".cluster", ".gpu", ".sys"],
            format!(".{other}"),
        )),
    }
}

fn expect_scope(stream: &mut PtxTokenStream) -> Result<(Scope, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let scope = scope_from_directive(&directive, span.clone())?;
    Ok((scope, span))
}

impl PtxParser for Scope {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        Ok(expect_scope(stream)?.0)
    }
}

impl PtxParser for ProxyKind {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "alias" => Ok(ProxyKind::Alias),
            "async" => {
                if stream
                    .check(|token| matches!(token, PtxToken::Directive(name) if name == "global"))
                {
                    stream.consume()?;
                    Ok(ProxyKind::AsyncGlobal)
                } else if stream
                    .check(|token| matches!(token, PtxToken::Directive(name) if name == "shared"))
                {
                    stream.consume()?;
                    stream.expect_double_colon()?;
                    let (target, target_span) = stream.expect_identifier()?;
                    match target.as_str() {
                        "cta" => Ok(ProxyKind::AsyncSharedCta),
                        "cluster" => Ok(ProxyKind::AsyncSharedCluster),
                        other => Err(unexpected_value(target_span, &["cta", "cluster"], other)),
                    }
                } else if stream
                    .consume_if(|token| matches!(token, PtxToken::Dot))
                    .is_some()
                {
                    let (identifier, identifier_span) = stream.expect_identifier()?;
                    match identifier.as_str() {
                        "global" => Ok(ProxyKind::AsyncGlobal),
                        "shared" => {
                            stream.expect_double_colon()?;
                            let (target, target_span) = stream.expect_identifier()?;
                            match target.as_str() {
                                "cta" => Ok(ProxyKind::AsyncSharedCta),
                                "cluster" => Ok(ProxyKind::AsyncSharedCluster),
                                other => {
                                    Err(unexpected_value(target_span, &["cta", "cluster"], other))
                                }
                            }
                        }
                        other => Err(unexpected_value(
                            identifier_span,
                            &["global", "shared"],
                            other,
                        )),
                    }
                } else {
                    Ok(ProxyKind::Async)
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

impl PtxParser for ProxySize {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (identifier, span) = stream.expect_identifier()?;
        if identifier != "size" {
            return Err(unexpected_value(span, &["size"], identifier));
        }

        stream.expect(&PtxToken::Equals)?;
        let (token, value_span) = stream.consume()?;
        let value_span = value_span.clone();
        match token {
            PtxToken::DecimalInteger(value) if value == "128" => Ok(ProxySize::B128),
            other => Err(unexpected_value(value_span, &["128"], format!("{other:?}"))),
        }
    }
}

impl PtxParser for Level {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "cta" => Ok(Level::Cta),
            "gl" => Ok(Level::Gl),
            "sys" => Ok(Level::Sys),
            other => Err(unexpected_value(
                span,
                &[".cta", ".gl", ".sys"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_optional_semantics(
    stream: &mut PtxTokenStream,
) -> Result<Option<Semantics>, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if is_semantics_name(name))) {
        Ok(Some(expect_semantics(stream)?.0))
    } else {
        Ok(None)
    }
}

fn parse_thread_fence(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let semantics = parse_optional_semantics(stream)?;
    let scope = Scope::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(Membar::ThreadFence(ThreadFence { semantics, scope }))
}

fn parse_thread_fence_sync_restrict(
    stream: &mut PtxTokenStream,
    semantics: Semantics,
    _semantics_span: Span,
) -> Result<Membar, PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "sync_restrict" {
        return Err(unexpected_value(
            span,
            &[".sync_restrict"],
            format!(".{directive}"),
        ));
    }

    stream.expect_double_colon()?;
    let (shared, shared_span) = stream.expect_identifier()?;
    if shared != "shared" {
        return Err(unexpected_value(shared_span, &["shared"], shared));
    }

    stream.expect_double_colon()?;
    let (level, level_span) = stream.expect_identifier()?;

    let variant = match (semantics, level.as_str()) {
        (Semantics::Acquire, "cluster") => ThreadFenceSyncRestrict::AcquireSharedCluster,
        (Semantics::Release, "cta") => ThreadFenceSyncRestrict::ReleaseSharedCta,
        (Semantics::Acquire, _) => return Err(unexpected_value(level_span, &["cluster"], level)),
        (Semantics::Release, _) => return Err(unexpected_value(level_span, &["cta"], level)),
        _ => unreachable!(),
    };

    let (scope, scope_span) = expect_scope(stream)?;
    if scope != Scope::Cluster {
        return Err(unexpected_value(
            scope_span,
            &[".cluster"],
            format!("{scope:?}"),
        ));
    }

    stream.expect(&PtxToken::Semicolon)?;
    Ok(Membar::ThreadFenceSyncRestrict(variant))
}

fn parse_operation_fence(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "op_restrict" {
        return Err(unexpected_value(
            span,
            &[".op_restrict"],
            format!(".{directive}"),
        ));
    }

    let (semantics, semantics_span, semantics_raw) = expect_semantics(stream)?;
    if semantics != Semantics::Release {
        return Err(unexpected_value(
            semantics_span,
            &[".release"],
            format!(".{semantics_raw}"),
        ));
    }

    let (scope, _) = expect_scope(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Membar::OperationFence(OperationFence { scope }))
}

fn parse_proxy_fence(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let kind = ProxyKind::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(Membar::ProxyFence(ProxyFence { kind }))
}

fn parse_proxy_tensormap(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "tensormap" {
        return Err(unexpected_value(
            span,
            &[".tensormap"],
            format!(".{directive}"),
        ));
    }

    stream.expect_double_colon()?;
    let (identifier, identifier_span) = stream.expect_identifier()?;
    if identifier != "generic" {
        return Err(unexpected_value(identifier_span, &["generic"], identifier));
    }

    let (semantics, semantics_span, semantics_raw) = expect_semantics(stream)?;
    match semantics {
        Semantics::Release => {
            let (scope, _) = expect_scope(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Membar::ProxyTensormapRelease(ProxyTensormapRelease {
                scope,
            }))
        }
        Semantics::Acquire => {
            let (scope, _) = expect_scope(stream)?;
            let address = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = ProxySize::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Membar::ProxyTensormapAcquire(ProxyTensormapAcquire {
                scope,
                address,
                size,
            }))
        }
        _ => Err(unexpected_value(
            semantics_span,
            &[".acquire", ".release"],
            format!(".{semantics_raw}"),
        )),
    }
}

fn parse_proxy_async(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "async" {
        return Err(unexpected_value(span, &[".async"], format!(".{directive}")));
    }

    stream.expect_double_colon()?;
    let (identifier, identifier_span) = stream.expect_identifier()?;
    if identifier != "generic" {
        return Err(unexpected_value(identifier_span, &["generic"], identifier));
    }

    let (semantics, semantics_span, semantics_raw) = expect_semantics(stream)?;
    if !matches!(semantics, Semantics::Acquire | Semantics::Release) {
        return Err(unexpected_value(
            semantics_span,
            &[".acquire", ".release"],
            format!(".{semantics_raw}"),
        ));
    }

    let (directive, span) = stream.expect_directive()?;
    if directive != "sync_restrict" {
        return Err(unexpected_value(
            span,
            &[".sync_restrict"],
            format!(".{directive}"),
        ));
    }

    stream.expect_double_colon()?;
    let (shared, shared_span) = stream.expect_identifier()?;
    if shared != "shared" {
        return Err(unexpected_value(shared_span, &["shared"], shared));
    }

    stream.expect_double_colon()?;
    let (level, level_span) = stream.expect_identifier()?;

    let variant = match (semantics, level.as_str()) {
        (Semantics::Acquire, "cluster") => ProxyAsync::AcquireSharedCluster,
        (Semantics::Release, "cta") => ProxyAsync::ReleaseSharedCta,
        (Semantics::Acquire, _) => return Err(unexpected_value(level_span, &["cluster"], level)),
        (Semantics::Release, _) => return Err(unexpected_value(level_span, &["cta"], level)),
        _ => unreachable!(),
    };

    let (scope, scope_span) = expect_scope(stream)?;
    if scope != Scope::Cluster {
        return Err(unexpected_value(
            scope_span,
            &[".cluster"],
            format!("{scope:?}"),
        ));
    }

    stream.expect(&PtxToken::Semicolon)?;
    Ok(Membar::ProxyAsync(variant))
}

fn parse_proxy(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "proxy" {
        return Err(unexpected_value(span, &[".proxy"], format!(".{directive}")));
    }

    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "tensormap")) {
        parse_proxy_tensormap(stream)
    } else if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "async")) {
        parse_proxy_async(stream)
    } else {
        parse_proxy_fence(stream)
    }
}

fn parse_membar_old_style(stream: &mut PtxTokenStream) -> Result<Membar, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "proxy")) {
        let (directive, span) = stream.expect_directive()?;
        if directive != "proxy" {
            return Err(unexpected_value(span, &[".proxy"], format!(".{directive}")));
        }

        let kind = ProxyKind::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Membar::OldStyleProxy(OldStyleProxy { kind }))
    } else {
        let level = Level::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Membar::OldStyleScope(OldStyleScope { level }))
    }
}

impl PtxParser for Membar {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "fence" => {
                if stream
                    .check(|token| matches!(token, PtxToken::Directive(name) if name == "proxy"))
                {
                    parse_proxy(stream)
                } else if stream.check(
                    |token| matches!(token, PtxToken::Directive(name) if name == "op_restrict"),
                ) {
                    parse_operation_fence(stream)
                } else if stream.check(
                    |token| matches!(token, PtxToken::Directive(name) if is_semantics_name(name)),
                ) {
                    let (semantics, semantics_span, semantics_raw) = expect_semantics(stream)?;
                    if stream
                        .check(|token| matches!(token, PtxToken::Directive(name) if name == "sync_restrict"))
                    {
                        if matches!(semantics, Semantics::Acquire | Semantics::Release) {
                            parse_thread_fence_sync_restrict(stream, semantics, semantics_span)
                        } else {
                            Err(unexpected_value(
                                semantics_span,
                                &[".acquire", ".release"],
                                format!(".{semantics_raw}"),
                            ))
                        }
                    } else {
                        let (scope, _) = expect_scope(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        Ok(Membar::ThreadFence(ThreadFence {
                            semantics: Some(semantics),
                            scope,
                        }))
                    }
                } else if stream.check(
                    |token| matches!(token, PtxToken::Directive(name) if is_scope_name(name)),
                ) {
                    parse_thread_fence(stream)
                } else {
                    Err(unexpected_value(
                        span,
                        &[
                            "fence{.sem}.scope",
                            "fence.acquire.sync_restrict::shared::cluster.cluster",
                            "fence.release.sync_restrict::shared::cta.cluster",
                            "fence.op_restrict.release.scope",
                            "fence.proxy.*",
                        ],
                        "fence",
                    ))
                }
            }
            "membar" => parse_membar_old_style(stream),
            other => Err(unexpected_value(span, &["fence", "membar"], other)),
        }
    }
}
