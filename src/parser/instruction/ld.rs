use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::*,
        instruction::ld::{
            CacheHint, CacheOperator, DataType, Destination, DestinationElement,
            DestinationElements, Eviction, Generic, Ld, Level1EvictionPriority,
            Level2EvictionPriority, Mmio, MmioStateSpace, ParamState, PrefetchSize, Scope, Scoped,
            ScopedStateSpace, SharedState, StateSpace, Vector, Volatile,
        },
    },
};

fn is_data_type_directive(value: &str) -> bool {
    matches!(
        value,
        "b8" | "b16"
            | "b32"
            | "b64"
            | "b128"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "s8"
            | "s16"
            | "s32"
            | "s64"
            | "f32"
            | "f64"
    )
}

fn parse_cache_operator(
    stream: &mut PtxTokenStream,
) -> Result<(CacheOperator, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let cache_operator = match modifier.as_str() {
        "ca" => CacheOperator::Ca,
        "cg" => CacheOperator::Cg,
        "cs" => CacheOperator::Cs,
        "lu" => CacheOperator::Lu,
        "cv" => CacheOperator::Cv,
        other => {
            return Err(unexpected_value(
                span,
                &[".ca", ".cg", ".cs", ".lu", ".cv"],
                format!(".{other}"),
            ));
        }
    };
    Ok((cache_operator, span))
}

fn parse_state_space(stream: &mut PtxTokenStream) -> Result<(StateSpace, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let state_space = match modifier.as_str() {
        "const" => StateSpace::Const,
        "global" => StateSpace::Global,
        "local" => StateSpace::Local,
        "param" => {
            if stream.check(|token| matches!(token, PtxToken::Colon)) {
                stream.expect_double_colon()?;
                let (target, target_span) = stream.expect_identifier()?;
                match target.as_str() {
                    "entry" => StateSpace::Param(ParamState::Entry),
                    "func" => StateSpace::Param(ParamState::Func),
                    other => return Err(unexpected_value(target_span, &["entry", "func"], other)),
                }
            } else {
                StateSpace::Param(ParamState::Func)
            }
        }
        "shared" => {
            stream.expect_double_colon()?;
            let (target, target_span) = stream.expect_identifier()?;
            match target.as_str() {
                "cta" => StateSpace::Shared(SharedState::Cta),
                "cluster" => StateSpace::Shared(SharedState::Cluster),
                other => return Err(unexpected_value(target_span, &["cta", "cluster"], other)),
            }
        }
        other => {
            return Err(unexpected_value(
                span,
                &[
                    ".const",
                    ".global",
                    ".local",
                    ".param::entry",
                    ".param::func",
                    ".shared::cta",
                    ".shared::cluster",
                ],
                format!(".{other}"),
            ));
        }
    };
    Ok((state_space, span))
}

fn parse_scoped_state_space(
    stream: &mut PtxTokenStream,
) -> Result<(ScopedStateSpace, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let state_space = match modifier.as_str() {
        "global" => ScopedStateSpace::Global,
        "shared" => {
            stream.expect_double_colon()?;
            let (target, target_span) = stream.expect_identifier()?;
            match target.as_str() {
                "cta" => ScopedStateSpace::Shared(SharedState::Cta),
                "cluster" => ScopedStateSpace::Shared(SharedState::Cluster),
                other => return Err(unexpected_value(target_span, &["cta", "cluster"], other)),
            }
        }
        other => {
            return Err(unexpected_value(
                span,
                &[".global", ".shared::cta", ".shared::cluster"],
                format!(".{other}"),
            ));
        }
    };
    Ok((state_space, span))
}

fn parse_mmio_state_space(
    stream: &mut PtxTokenStream,
) -> Result<(MmioStateSpace, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let state_space = match modifier.as_str() {
        "global" => MmioStateSpace::Global,
        other => return Err(unexpected_value(span, &[".global"], format!(".{other}"))),
    };
    Ok((state_space, span))
}

fn parse_scope(stream: &mut PtxTokenStream) -> Result<(Scope, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let scope = match modifier.as_str() {
        "cta" => Scope::Cta,
        "cluster" => Scope::Cluster,
        "gpu" => Scope::Gpu,
        "sys" => Scope::Sys,
        other => {
            return Err(unexpected_value(
                span,
                &[".cta", ".cluster", ".gpu", ".sys"],
                format!(".{other}"),
            ));
        }
    };
    Ok((scope, span))
}

fn parse_vector_modifier(stream: &mut PtxTokenStream) -> Result<(Vector, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let vector = match modifier.as_str() {
        "v2" => Vector::V2,
        "v4" => Vector::V4,
        "v8" => Vector::V8,
        other => {
            return Err(unexpected_value(
                span,
                &[".v2", ".v4", ".v8"],
                format!(".{other}"),
            ));
        }
    };
    Ok((vector, span))
}

fn parse_data_type(stream: &mut PtxTokenStream) -> Result<DataType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let data_type = match modifier.as_str() {
        "b8" => DataType::B8,
        "b16" => DataType::B16,
        "b32" => DataType::B32,
        "b64" => DataType::B64,
        "b128" => DataType::B128,
        "u8" => DataType::U8,
        "u16" => DataType::U16,
        "u32" => DataType::U32,
        "u64" => DataType::U64,
        "s8" => DataType::S8,
        "s16" => DataType::S16,
        "s32" => DataType::S32,
        "s64" => DataType::S64,
        "f32" => DataType::F32,
        "f64" => DataType::F64,
        other => {
            return Err(unexpected_value(
                span,
                &[
                    ".b8", ".b16", ".b32", ".b64", ".b128", ".u8", ".u16", ".u32", ".u64", ".s8",
                    ".s16", ".s32", ".s64", ".f32", ".f64",
                ],
                format!(".{other}"),
            ));
        }
    };
    Ok(data_type)
}

fn parse_level1_modifier(
    stream: &mut PtxTokenStream,
) -> Result<(Level1EvictionPriority, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    if modifier != "L1" {
        return Err(unexpected_value(span, &[".L1"], format!(".{modifier}")));
    }
    stream.expect_double_colon()?;
    let (name, name_span) = stream.expect_identifier()?;
    let level1 = match name.as_str() {
        "evict_normal" => Level1EvictionPriority::EvictNormal,
        "evict_unchanged" => Level1EvictionPriority::EvictUnchanged,
        "evict_first" => Level1EvictionPriority::EvictFirst,
        "evict_last" => Level1EvictionPriority::EvictLast,
        "no_allocate" => Level1EvictionPriority::NoAllocate,
        other => {
            return Err(unexpected_value(
                name_span,
                &[
                    "evict_normal",
                    "evict_unchanged",
                    "evict_first",
                    "evict_last",
                    "no_allocate",
                ],
                other,
            ));
        }
    };
    Ok((level1, span))
}

enum L2Modifier {
    Eviction(Level2EvictionPriority),
    CacheHint(CacheHint),
    Prefetch(PrefetchSize),
}

fn parse_level2_modifier(stream: &mut PtxTokenStream) -> Result<(L2Modifier, Span), PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    if modifier != "L2" {
        return Err(unexpected_value(span, &[".L2"], format!(".{modifier}")));
    }
    stream.expect_double_colon()?;
    let modifier = {
        let (token, name_span) = stream.peek()?;
        match token {
            PtxToken::Identifier(name) => {
                let value = name.clone();
                stream.consume()?;
                match value.as_str() {
                    "evict_normal" => L2Modifier::Eviction(Level2EvictionPriority::EvictNormal),
                    "evict_first" => L2Modifier::Eviction(Level2EvictionPriority::EvictFirst),
                    "evict_last" => L2Modifier::Eviction(Level2EvictionPriority::EvictLast),
                    "cache_hint" => L2Modifier::CacheHint(CacheHint::L2CacheHint),
                    other => {
                        return Err(unexpected_value(
                            name_span.clone(),
                            &[
                                "evict_normal",
                                "evict_first",
                                "evict_last",
                                "cache_hint",
                                "64B",
                                "128B",
                                "256B",
                            ],
                            other,
                        ));
                    }
                }
            }
            PtxToken::DecimalInteger(number) => {
                let digits = number.clone();
                let digits_span = name_span.clone();
                stream.consume()?;
                let (suffix, suffix_span) = stream.expect_identifier()?;
                if suffix != "B" {
                    return Err(unexpected_value(suffix_span, &["B"], suffix));
                }
                match digits.as_str() {
                    "64" => L2Modifier::Prefetch(PrefetchSize::L264B),
                    "128" => L2Modifier::Prefetch(PrefetchSize::L2128B),
                    "256" => L2Modifier::Prefetch(PrefetchSize::L2256B),
                    other => {
                        return Err(unexpected_value(
                            digits_span,
                            &["64B", "128B", "256B"],
                            other,
                        ));
                    }
                }
            }
            _ => {
                return Err(unexpected_value(
                    name_span.clone(),
                    &[
                        "evict_normal",
                        "evict_first",
                        "evict_last",
                        "cache_hint",
                        "64B",
                        "128B",
                        "256B",
                    ],
                    format!("{token:?}"),
                ));
            }
        }
    };
    Ok((modifier, span))
}

fn parse_destination_element(
    stream: &mut PtxTokenStream,
) -> Result<DestinationElement, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Register(_))) {
        let (name, _) = stream.expect_register()?;
        return Ok(DestinationElement::Register(RegisterOperand::Single(name)));
    }

    let (identifier, span) = stream.expect_identifier()?;
    if identifier == "_" {
        Ok(DestinationElement::Sink)
    } else {
        Err(unexpected_value(span, &["_", "register"], identifier))
    }
}

fn parse_destination_elements(
    stream: &mut PtxTokenStream,
    vector: Vector,
) -> Result<DestinationElements, PtxParseError> {
    stream.expect(&PtxToken::LBrace)?;
    let result = match vector {
        Vector::V2 => {
            let first = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let second = parse_destination_element(stream)?;
            DestinationElements::V2([first, second])
        }
        Vector::V4 => {
            let first = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let second = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let third = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let fourth = parse_destination_element(stream)?;
            DestinationElements::V4([first, second, third, fourth])
        }
        Vector::V8 => {
            let e0 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e1 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e2 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e3 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e4 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e5 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e6 = parse_destination_element(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let e7 = parse_destination_element(stream)?;
            DestinationElements::V8([e0, e1, e2, e3, e4, e5, e6, e7])
        }
    };
    stream.expect(&PtxToken::RBrace)?;
    Ok(result)
}

fn parse_destination(
    stream: &mut PtxTokenStream,
    vector: Option<Vector>,
) -> Result<Destination, PtxParseError> {
    match vector {
        Some(vector) => {
            if !stream.check(|token| matches!(token, PtxToken::LBrace)) {
                let (token, span) = stream.peek()?;
                return Err(unexpected_value(span.clone(), &["{"], format!("{token:?}")));
            }
            Ok(Destination::Vector(parse_destination_elements(
                stream, vector,
            )?))
        }
        None => {
            if stream.check(|token| matches!(token, PtxToken::LBrace)) {
                let (token, span) = stream.peek()?;
                return Err(unexpected_value(
                    span.clone(),
                    &["register"],
                    format!("{token:?}"),
                ));
            }
            Ok(Destination::Scalar(RegisterOperand::parse(stream)?))
        }
    }
}

fn parse_generic_or_eviction(stream: &mut PtxTokenStream) -> Result<Ld, PtxParseError> {
    let mut weak = None;
    let mut state_space: Option<(StateSpace, Span)> = None;
    let mut cache_operator: Option<(CacheOperator, Span)> = None;
    let mut level1: Option<(Level1EvictionPriority, Span)> = None;
    let mut level2: Option<(Level2EvictionPriority, Span)> = None;
    let mut cache_hint: Option<(CacheHint, Span)> = None;
    let mut prefetch_size: Option<(PrefetchSize, Span)> = None;
    let mut vector: Option<(Vector, Span)> = None;

    while let Some((directive, span)) = peek_directive(stream)? {
        match directive.as_str() {
            "weak" => {
                stream.consume()?;
                if weak.is_some() {
                    return Err(unexpected_value(span, &["unique .weak"], ".weak"));
                }
                weak = Some(span);
            }
            "const" | "global" | "local" | "param" | "shared" => {
                if state_space.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one state space"],
                        format!(".{directive}"),
                    ));
                }
                let (value, state_span) = parse_state_space(stream)?;
                state_space = Some((value, state_span));
            }
            "ca" | "cg" | "cs" | "lu" | "cv" => {
                if cache_operator.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one cache operator"],
                        format!(".{directive}"),
                    ));
                }
                let (value, cache_span) = parse_cache_operator(stream)?;
                cache_operator = Some((value, cache_span));
            }
            "L1" => {
                if level1.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one L1 eviction priority"],
                        ".L1",
                    ));
                }
                let (value, level_span) = parse_level1_modifier(stream)?;
                level1 = Some((value, level_span));
            }
            "L2" => {
                let (modifier, modifier_span) = parse_level2_modifier(stream)?;
                match modifier {
                    L2Modifier::Eviction(value) => {
                        if level2.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one L2 eviction priority"],
                                ".L2",
                            ));
                        }
                        level2 = Some((value, modifier_span));
                    }
                    L2Modifier::CacheHint(value) => {
                        if cache_hint.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one cache hint"],
                                ".L2",
                            ));
                        }
                        cache_hint = Some((value, modifier_span));
                    }
                    L2Modifier::Prefetch(value) => {
                        if prefetch_size.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one prefetch size"],
                                ".L2",
                            ));
                        }
                        prefetch_size = Some((value, modifier_span));
                    }
                }
            }
            "v2" | "v4" | "v8" => {
                if vector.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one vector modifier"],
                        format!(".{directive}"),
                    ));
                }
                let (value, vector_span) = parse_vector_modifier(stream)?;
                vector = Some((value, vector_span));
            }
            other if is_data_type_directive(other) => break,
            other => {
                return Err(unexpected_value(
                    span.clone(),
                    &[
                        ".weak",
                        ".const",
                        ".global",
                        ".local",
                        ".param::entry",
                        ".param::func",
                        ".shared::cta",
                        ".shared::cluster",
                        ".ca",
                        ".cg",
                        ".cs",
                        ".lu",
                        ".cv",
                        ".L1::<eviction>",
                        ".L2::<modifier>",
                        ".v2",
                        ".v4",
                        ".v8",
                        "data type",
                    ],
                    format!(".{other}"),
                ));
            }
        }
    }

    let data_type = parse_data_type(stream)?;
    let destination = parse_destination(stream, vector.as_ref().map(|(v, _)| *v))?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;

    let unified = if let Some((directive, span)) = peek_directive(stream)? {
        if directive == "unified" {
            stream.consume()?;
            true
        } else {
            return Err(unexpected_value(
                span,
                &[".unified", "comma", "semicolon"],
                format!(".{directive}"),
            ));
        }
    } else {
        false
    };

    let cache_policy = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(RegisterOperand::parse(stream)?)
    } else {
        None
    };

    stream.expect(&PtxToken::Semicolon)?;

    if level1.is_some() || level2.is_some() {
        if let Some((_, span)) = cache_operator {
            return Err(unexpected_value(
                span,
                &["cache operator not allowed with eviction priorities"],
                "cache operator",
            ));
        }
        return Ok(Ld::Eviction(Eviction {
            weak: weak.is_some(),
            state_space: state_space.map(|(value, _)| value),
            level1: level1.map(|(value, _)| value),
            level2: level2.map(|(value, _)| value),
            cache_hint: cache_hint.map(|(value, _)| value),
            prefetch_size: prefetch_size.map(|(value, _)| value),
            vector: vector.map(|(value, _)| value),
            data_type,
            destination,
            address,
            unified,
            cache_policy,
        }));
    }

    Ok(Ld::Generic(Generic {
        weak: weak.is_some(),
        state_space: state_space.map(|(value, _)| value),
        cache_operator: cache_operator.map(|(value, _)| value),
        cache_hint: cache_hint.map(|(value, _)| value),
        prefetch_size: prefetch_size.map(|(value, _)| value),
        vector: vector.map(|(value, _)| value),
        data_type,
        destination,
        address,
        unified,
        cache_policy,
    }))
}

fn parse_volatile(stream: &mut PtxTokenStream) -> Result<Ld, PtxParseError> {
    expect_directive_value(stream, "volatile")?;

    let mut state_space: Option<(ScopedStateSpace, Span)> = None;
    let mut prefetch_size: Option<(PrefetchSize, Span)> = None;
    let mut vector: Option<(Vector, Span)> = None;

    while let Some((directive, span)) = peek_directive(stream)? {
        match directive.as_str() {
            "global" | "shared" => {
                if state_space.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one state space"],
                        format!(".{directive}"),
                    ));
                }
                let (value, state_span) = parse_scoped_state_space(stream)?;
                state_space = Some((value, state_span));
            }
            "L2" => {
                let (modifier, modifier_span) = parse_level2_modifier(stream)?;
                match modifier {
                    L2Modifier::Prefetch(value) => {
                        if prefetch_size.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one prefetch size"],
                                ".L2",
                            ));
                        }
                        prefetch_size = Some((value, modifier_span));
                    }
                    L2Modifier::Eviction(_) | L2Modifier::CacheHint(_) => {
                        return Err(unexpected_value(
                            modifier_span,
                            &["prefetch size modifier"],
                            ".L2::<modifier>",
                        ));
                    }
                }
            }
            "v2" | "v4" | "v8" => {
                if vector.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one vector modifier"],
                        format!(".{directive}"),
                    ));
                }
                let (value, vector_span) = parse_vector_modifier(stream)?;
                vector = Some((value, vector_span));
            }
            other if is_data_type_directive(other) => break,
            other => {
                return Err(unexpected_value(
                    span.clone(),
                    &[
                        ".global",
                        ".shared::cta",
                        ".shared::cluster",
                        ".L2::<prefetch>",
                        ".v2",
                        ".v4",
                        ".v8",
                        "data type",
                    ],
                    format!(".{other}"),
                ));
            }
        }
    }

    let data_type = parse_data_type(stream)?;
    let destination = parse_destination(stream, vector.as_ref().map(|(v, _)| *v))?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;

    if let Some((directive, span)) = peek_directive(stream)? {
        return Err(unexpected_value(span, &[";"], format!(".{directive}")));
    }

    if stream.check(|token| matches!(token, PtxToken::Comma)) {
        let (token, span) = stream.peek()?;
        return Err(unexpected_value(span.clone(), &[";"], format!("{token:?}")));
    }

    stream.expect(&PtxToken::Semicolon)?;

    Ok(Ld::Volatile(Volatile {
        state_space: state_space.map(|(value, _)| value),
        prefetch_size: prefetch_size.map(|(value, _)| value),
        vector: vector.map(|(value, _)| value),
        data_type,
        destination,
        address,
    }))
}

fn parse_scoped_load(
    stream: &mut PtxTokenStream,
    variant: fn(Scoped) -> Ld,
) -> Result<Ld, PtxParseError> {
    let (scope, _) = parse_scope(stream)?;

    let mut state_space: Option<(ScopedStateSpace, Span)> = None;
    let mut level1: Option<(Level1EvictionPriority, Span)> = None;
    let mut level2: Option<(Level2EvictionPriority, Span)> = None;
    let mut cache_hint: Option<(CacheHint, Span)> = None;
    let mut prefetch_size: Option<(PrefetchSize, Span)> = None;
    let mut vector: Option<(Vector, Span)> = None;

    while let Some((directive, span)) = peek_directive(stream)? {
        match directive.as_str() {
            "global" | "shared" => {
                if state_space.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one state space"],
                        format!(".{directive}"),
                    ));
                }
                let (value, state_span) = parse_scoped_state_space(stream)?;
                state_space = Some((value, state_span));
            }
            "L1" => {
                if level1.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one L1 eviction priority"],
                        ".L1",
                    ));
                }
                let (value, level_span) = parse_level1_modifier(stream)?;
                level1 = Some((value, level_span));
            }
            "L2" => {
                let (modifier, modifier_span) = parse_level2_modifier(stream)?;
                match modifier {
                    L2Modifier::Eviction(value) => {
                        if level2.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one L2 eviction priority"],
                                ".L2",
                            ));
                        }
                        level2 = Some((value, modifier_span));
                    }
                    L2Modifier::CacheHint(value) => {
                        if cache_hint.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one cache hint"],
                                ".L2",
                            ));
                        }
                        cache_hint = Some((value, modifier_span));
                    }
                    L2Modifier::Prefetch(value) => {
                        if prefetch_size.is_some() {
                            return Err(unexpected_value(
                                modifier_span,
                                &["at most one prefetch size"],
                                ".L2",
                            ));
                        }
                        prefetch_size = Some((value, modifier_span));
                    }
                }
            }
            "v2" | "v4" | "v8" => {
                if vector.is_some() {
                    return Err(unexpected_value(
                        span.clone(),
                        &["at most one vector modifier"],
                        format!(".{directive}"),
                    ));
                }
                let (value, vector_span) = parse_vector_modifier(stream)?;
                vector = Some((value, vector_span));
            }
            other if is_data_type_directive(other) => break,
            other => {
                return Err(unexpected_value(
                    span.clone(),
                    &[
                        ".global",
                        ".shared::cta",
                        ".shared::cluster",
                        ".L1::<eviction>",
                        ".L2::<modifier>",
                        ".v2",
                        ".v4",
                        ".v8",
                        "data type",
                    ],
                    format!(".{other}"),
                ));
            }
        }
    }

    let data_type = parse_data_type(stream)?;
    let destination = parse_destination(stream, vector.as_ref().map(|(v, _)| *v))?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;

    if let Some((directive, span)) = peek_directive(stream)? {
        return Err(unexpected_value(
            span,
            &["comma", "semicolon"],
            format!(".{directive}"),
        ));
    }

    let cache_policy = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(RegisterOperand::parse(stream)?)
    } else {
        None
    };

    stream.expect(&PtxToken::Semicolon)?;

    Ok(variant(Scoped {
        scope,
        state_space: state_space.map(|(value, _)| value),
        level1: level1.map(|(value, _)| value),
        level2: level2.map(|(value, _)| value),
        cache_hint: cache_hint.map(|(value, _)| value),
        prefetch_size: prefetch_size.map(|(value, _)| value),
        vector: vector.map(|(value, _)| value),
        data_type,
        destination,
        address,
        cache_policy,
    }))
}

fn parse_relaxed(stream: &mut PtxTokenStream) -> Result<Ld, PtxParseError> {
    expect_directive_value(stream, "relaxed")?;
    parse_scoped_load(stream, Ld::Relaxed)
}

fn parse_acquire(stream: &mut PtxTokenStream) -> Result<Ld, PtxParseError> {
    expect_directive_value(stream, "acquire")?;
    parse_scoped_load(stream, Ld::Acquire)
}

fn parse_mmio(stream: &mut PtxTokenStream) -> Result<Ld, PtxParseError> {
    expect_directive_value(stream, "mmio")?;
    expect_directive_value(stream, "relaxed")?;
    expect_directive_value(stream, "sys")?;

    let state_space = match peek_directive(stream)? {
        Some((directive, _)) if directive == "global" => Some(parse_mmio_state_space(stream)?.0),
        Some((directive, _)) if is_data_type_directive(&directive) => None,
        Some((directive, span)) => {
            return Err(unexpected_value(
                span,
                &[".global", "data type"],
                format!(".{directive}"),
            ));
        }
        None => None,
    };

    let data_type = parse_data_type(stream)?;
    let destination = parse_destination(stream, None)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;

    if let Some((directive, span)) = peek_directive(stream)? {
        return Err(unexpected_value(span, &[";"], format!(".{directive}")));
    }

    stream.expect(&PtxToken::Semicolon)?;

    Ok(Ld::Mmio(Mmio {
        state_space,
        data_type,
        destination,
        address,
    }))
}

impl PtxParser for Ld {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "ld" {
            return Err(unexpected_value(span, &["ld"], opcode));
        }

        match peek_directive(stream)? {
            Some((directive, _)) if directive == "volatile" => parse_volatile(stream),
            Some((directive, _)) if directive == "relaxed" => parse_relaxed(stream),
            Some((directive, _)) if directive == "acquire" => parse_acquire(stream),
            Some((directive, _)) if directive == "mmio" => parse_mmio(stream),
            _ => parse_generic_or_eviction(stream),
        }
    }
}
