use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::cp::*},
};

#[derive(Clone)]
enum TrailingArgument {
    Operand {
        value: Operand,
        span: Span,
    },
    Predicate {
        value: PredicateRegister,
        span: Span,
    },
}

impl TrailingArgument {
    fn span(&self) -> Span {
        match self {
            TrailingArgument::Operand { span, .. } | TrailingArgument::Predicate { span, .. } => {
                span.clone()
            }
        }
    }
}

enum L2Modifier {
    CacheHint,
    Prefetch(CpPrefetchSize),
}

impl PtxParser for CpSharedSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        if directive != "shared" {
            return Err(unexpected_value(
                span,
                &[".shared", ".shared::cta"],
                format!(".{directive}"),
            ));
        }

        if stream.check(|token| matches!(token, PtxToken::Colon)) {
            stream.expect_double_colon()?;
            let (target, target_span) = stream.expect_identifier()?;
            match target.as_str() {
                "cta" => Ok(CpSharedSpace::Cta),
                other => Err(unexpected_value(target_span, &["cta"], other)),
            }
        } else {
            Ok(CpSharedSpace::Default)
        }
    }
}

fn parse_l2_modifier(stream: &mut PtxTokenStream) -> Result<(L2Modifier, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    if directive != "L2" {
        return Err(unexpected_value(span, &[".L2"], format!(".{directive}")));
    }

    stream.expect_double_colon()?;
    let (token, token_span) = stream.peek()?;
    match token {
        PtxToken::Identifier(name) if name == "cache_hint" => {
            stream.consume()?;
            Ok((L2Modifier::CacheHint, span))
        }
        PtxToken::Identifier(name) => Err(unexpected_value(
            token_span.clone(),
            &["cache_hint", "64B", "128B", "256B"],
            name.clone(),
        )),
        PtxToken::DecimalInteger(digits) => {
            let digits = digits.clone();
            let digits_span = token_span.clone();
            stream.consume()?;
            let (suffix, suffix_span) = stream.expect_identifier()?;
            if suffix != "B" {
                return Err(unexpected_value(suffix_span, &["B"], suffix));
            }

            let modifier = match digits.as_str() {
                "64" => L2Modifier::Prefetch(CpPrefetchSize::L264B),
                "128" => L2Modifier::Prefetch(CpPrefetchSize::L2128B),
                "256" => L2Modifier::Prefetch(CpPrefetchSize::L2256B),
                other => {
                    return Err(unexpected_value(
                        digits_span,
                        &["64B", "128B", "256B"],
                        other,
                    ));
                }
            };
            Ok((modifier, span))
        }
        _ => Err(unexpected_value(
            token_span.clone(),
            &["cache_hint", "64B", "128B", "256B"],
            format!("{token:?}"),
        )),
    }
}

fn parse_optional_l2_modifiers(
    stream: &mut PtxTokenStream,
) -> Result<(Option<CpCacheHint>, Option<CpPrefetchSize>), PtxParseError> {
    let mut cache_hint = None;
    let mut prefetch_size = None;

    loop {
        match peek_directive(stream)? {
            Some((name, _)) if name == "L2" => {
                let (modifier, span) = parse_l2_modifier(stream)?;
                match modifier {
                    L2Modifier::CacheHint => {
                        if cache_hint.is_some() {
                            return Err(invalid_literal(
                                span,
                                "duplicate .L2::cache_hint qualifier",
                            ));
                        }
                        cache_hint = Some(CpCacheHint::L2CacheHint);
                    }
                    L2Modifier::Prefetch(size) => {
                        if prefetch_size.is_some() {
                            return Err(invalid_literal(span, "duplicate .L2 prefetch qualifier"));
                        }
                        prefetch_size = Some(size);
                    }
                }
            }
            _ => break,
        }
    }

    Ok((cache_hint, prefetch_size))
}

fn parse_trailing_argument(stream: &mut PtxTokenStream) -> Result<TrailingArgument, PtxParseError> {
    let span = stream.peek()?.1.clone();
    if stream.check(|token| matches!(token, PtxToken::Register(name) if name.starts_with("%p"))) {
        Ok(TrailingArgument::Predicate {
            value: PredicateRegister::parse(stream)?,
            span,
        })
    } else {
        Ok(TrailingArgument::Operand {
            value: Operand::parse(stream)?,
            span,
        })
    }
}

fn parse_trailing_arguments(
    stream: &mut PtxTokenStream,
) -> Result<Vec<TrailingArgument>, PtxParseError> {
    let mut args = Vec::new();
    while stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        let argument = parse_trailing_argument(stream)?;
        if args.len() >= 2 {
            return Err(invalid_literal(
                argument.span(),
                "cp.async accepts at most two trailing operands",
            ));
        }
        args.push(argument);
    }
    Ok(args)
}

fn operand_to_register(operand: &Operand, span: Span) -> Result<RegisterOperand, PtxParseError> {
    if let Operand::Register(register) = operand {
        Ok(register.clone())
    } else {
        Err(unexpected_value(
            span,
            &["register operand"],
            format!("{operand:?}"),
        ))
    }
}

fn ensure_cache_policy_allowed(
    cache_hint: Option<CpCacheHint>,
    span: Span,
) -> Result<(), PtxParseError> {
    if cache_hint.is_some() {
        Ok(())
    } else {
        Err(invalid_literal(
            span,
            "cache policy requires .L2::cache_hint",
        ))
    }
}

enum TrailingKind {
    WithSrc {
        source_size: Option<Operand>,
        cache_policy: Option<RegisterOperand>,
    },
    Ignore {
        ignore_src: PredicateRegister,
        cache_policy: Option<RegisterOperand>,
    },
}

fn classify_trailing(
    args: &[TrailingArgument],
    cache_hint: Option<CpCacheHint>,
) -> Result<TrailingKind, PtxParseError> {
    match args {
        [] => Ok(TrailingKind::WithSrc {
            source_size: None,
            cache_policy: None,
        }),
        [TrailingArgument::Predicate { value, .. }] => Ok(TrailingKind::Ignore {
            ignore_src: value.clone(),
            cache_policy: None,
        }),
        [TrailingArgument::Operand { value, span }] => {
            if matches!(value, Operand::Register(_)) && cache_hint.is_some() {
                let register = operand_to_register(value, span.clone())?;
                ensure_cache_policy_allowed(cache_hint, span.clone())?;
                Ok(TrailingKind::WithSrc {
                    source_size: None,
                    cache_policy: Some(register),
                })
            } else {
                Ok(TrailingKind::WithSrc {
                    source_size: Some(value.clone()),
                    cache_policy: None,
                })
            }
        }
        [
            TrailingArgument::Operand { value: first, .. },
            TrailingArgument::Operand {
                value: second,
                span,
            },
        ] => {
            let register = operand_to_register(second, span.clone())?;
            ensure_cache_policy_allowed(cache_hint, span.clone())?;
            Ok(TrailingKind::WithSrc {
                source_size: Some(first.clone()),
                cache_policy: Some(register),
            })
        }
        [
            TrailingArgument::Predicate { value, .. },
            TrailingArgument::Operand {
                value: second,
                span,
            },
        ] => {
            let register = operand_to_register(second, span.clone())?;
            ensure_cache_policy_allowed(cache_hint, span.clone())?;
            Ok(TrailingKind::Ignore {
                ignore_src: value.clone(),
                cache_policy: Some(register),
            })
        }
        [
            TrailingArgument::Operand { span, .. },
            TrailingArgument::Predicate { .. },
        ] => Err(unexpected_value(
            span.clone(),
            &["register operand"],
            "predicate register",
        )),
        [
            TrailingArgument::Predicate { span, .. },
            TrailingArgument::Predicate { .. },
        ] => Err(unexpected_value(
            span.clone(),
            &["register operand"],
            "predicate register",
        )),
        _ => unreachable!("parse_trailing_arguments limits the argument count"),
    }
}

fn parse_ca_instruction(stream: &mut PtxTokenStream) -> Result<CpOpcode, PtxParseError> {
    let shared_space = CpSharedSpace::parse(stream)?;
    expect_directive_value(stream, "global")?;
    let (cache_hint, prefetch_size) = parse_optional_l2_modifiers(stream)?;

    let destination = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let copy_size = parse_ca_copy_size(stream)?;

    let trailing = parse_trailing_arguments(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    match classify_trailing(&trailing, cache_hint)? {
        TrailingKind::WithSrc {
            source_size,
            cache_policy,
        } => Ok(CpOpcode::AsyncCaWithSrcSize(CpAsyncCaWithSrcSize {
            shared_space,
            cache_hint,
            prefetch_size,
            destination,
            source,
            copy_size,
            source_size,
            cache_policy,
        })),
        TrailingKind::Ignore {
            ignore_src,
            cache_policy,
        } => Ok(CpOpcode::AsyncCaIgnoreSrc(CpAsyncCaIgnoreSrc {
            shared_space,
            cache_hint,
            prefetch_size,
            destination,
            source,
            copy_size,
            ignore_src: Some(ignore_src),
            cache_policy,
        })),
    }
}

fn parse_cg_instruction(stream: &mut PtxTokenStream) -> Result<CpOpcode, PtxParseError> {
    let shared_space = CpSharedSpace::parse(stream)?;
    expect_directive_value(stream, "global")?;
    let (cache_hint, prefetch_size) = parse_optional_l2_modifiers(stream)?;

    let destination = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    expect_cg_copy_size(stream)?;

    let trailing = parse_trailing_arguments(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    match classify_trailing(&trailing, cache_hint)? {
        TrailingKind::WithSrc {
            source_size,
            cache_policy,
        } => Ok(CpOpcode::AsyncCgWithSrcSize(CpAsyncCgWithSrcSize {
            shared_space,
            cache_hint,
            prefetch_size,
            destination,
            source,
            source_size,
            cache_policy,
        })),
        TrailingKind::Ignore {
            ignore_src,
            cache_policy,
        } => Ok(CpOpcode::AsyncCgIgnoreSrc(CpAsyncCgIgnoreSrc {
            shared_space,
            cache_hint,
            prefetch_size,
            destination,
            source,
            ignore_src: Some(ignore_src),
            cache_policy,
        })),
    }
}

fn parse_ca_copy_size(stream: &mut PtxTokenStream) -> Result<CpCopySize, PtxParseError> {
    let (value, span) = parse_u16_literal(stream)?;
    match value {
        4 => Ok(CpCopySize::Bytes4),
        8 => Ok(CpCopySize::Bytes8),
        16 => Ok(CpCopySize::Bytes16),
        other => Err(unexpected_value(span, &["4", "8", "16"], other.to_string())),
    }
}

fn expect_cg_copy_size(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    let (value, span) = parse_u16_literal(stream)?;
    if value == 16 {
        Ok(())
    } else {
        Err(unexpected_value(span, &["16"], value.to_string()))
    }
}

impl PtxParser for CpOpcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "cp")?;
        expect_directive_value(stream, "async")?;

        let (cache_operator, span) = stream.expect_directive()?;
        match cache_operator.as_str() {
            "ca" => parse_ca_instruction(stream),
            "cg" => parse_cg_instruction(stream),
            other => Err(unexpected_value(span, &[".ca", ".cg"], format!(".{other}"))),
        }
    }
}
