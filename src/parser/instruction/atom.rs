use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span, peek_directive, unexpected_value},
    r#type::{
        common::{AddressOperand, RegisterOperand},
        instruction::atom::{
            AddNoFtz, Atom, CacheHint, CompareSwap, CompareSwapVariant, DataType, Exchange128,
            HalfWordType, NoFtzType, PackedType, Scalar, ScalarOperation, Scope, Semantics,
            SharedSpace, StateSpace, Vec16Registers, Vec32Registers, VectorAdd32, VectorHalf,
            VectorOperation, VectorPacked, VectorStateSpace,
        },
    },
};

fn parse_semantics(stream: &mut PtxTokenStream) -> Result<(Semantics, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let semantics = match directive.as_str() {
        "relaxed" => Semantics::Relaxed,
        "acquire" => Semantics::Acquire,
        "release" => Semantics::Release,
        "acq_rel" => Semantics::AcqRel,
        other => {
            return Err(unexpected_value(
                span,
                &[".relaxed", ".acquire", ".release", ".acq_rel"],
                format!(".{other}"),
            ));
        }
    };
    Ok((semantics, span))
}

fn parse_scope(stream: &mut PtxTokenStream) -> Result<(Scope, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let scope = match directive.as_str() {
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

fn parse_state_space(stream: &mut PtxTokenStream) -> Result<(StateSpace, Span), PtxParseError> {
    let (directive, mut span) = stream.expect_directive()?;
    let state_space = match directive.as_str() {
        "global" => StateSpace::Global,
        "shared" => {
            stream.expect_double_colon()?;
            let (identifier, identifier_span) = stream.expect_identifier()?;
            span.end = identifier_span.end;
            match identifier.as_str() {
                "cta" => StateSpace::Shared(SharedSpace::Cta),
                "cluster" => StateSpace::Shared(SharedSpace::Cluster),
                other => {
                    return Err(unexpected_value(
                        identifier_span,
                        &["cta", "cluster"],
                        other,
                    ));
                }
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

fn parse_cache_hint(
    stream: &mut PtxTokenStream,
) -> Result<Option<(CacheHint, Span)>, PtxParseError> {
    if let Some((value, _)) = peek_directive(stream)? {
        if value.as_str() == "L2" {
            let (_, mut hint_span) = stream.expect_directive()?;
            stream.expect_double_colon()?;
            let (identifier, identifier_span) = stream.expect_identifier()?;
            hint_span.end = identifier_span.end;
            let cache_hint = match identifier.as_str() {
                "cache_hint" => CacheHint::L2CacheHint,
                other => return Err(unexpected_value(identifier_span, &["cache_hint"], other)),
            };
            return Ok(Some((cache_hint, hint_span)));
        }
    }

    Ok(None)
}

fn parse_data_type(stream: &mut PtxTokenStream) -> Result<(DataType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let data_type = match directive.as_str() {
        "b32" => DataType::B32,
        "b64" => DataType::B64,
        "u32" => DataType::U32,
        "u64" => DataType::U64,
        "s32" => DataType::S32,
        "s64" => DataType::S64,
        "f32" => DataType::F32,
        "f64" => DataType::F64,
        other => {
            return Err(unexpected_value(
                span,
                &[
                    ".b32", ".b64", ".u32", ".u64", ".s32", ".s64", ".f32", ".f64",
                ],
                format!(".{other}"),
            ));
        }
    };

    Ok((data_type, span))
}

fn parse_noftz_type(stream: &mut PtxTokenStream) -> Result<(NoFtzType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let ty = match directive.as_str() {
        "f16" => NoFtzType::F16,
        "f16x2" => NoFtzType::F16x2,
        "bf16" => NoFtzType::Bf16,
        "bf16x2" => NoFtzType::Bf16x2,
        other => {
            return Err(unexpected_value(
                span,
                &[".f16", ".f16x2", ".bf16", ".bf16x2"],
                format!(".{other}"),
            ));
        }
    };
    Ok((ty, span))
}

fn parse_vector_operation(name: &str, span: Span) -> Result<VectorOperation, PtxParseError> {
    match name {
        "add" => Ok(VectorOperation::Add),
        "min" => Ok(VectorOperation::Min),
        "max" => Ok(VectorOperation::Max),
        other => Err(unexpected_value(
            span,
            &[".add", ".min", ".max"],
            format!(".{other}"),
        )),
    }
}

fn parse_half_word_type(
    stream: &mut PtxTokenStream,
) -> Result<(HalfWordType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let ty = match directive.as_str() {
        "f16" => HalfWordType::F16,
        "bf16" => HalfWordType::Bf16,
        other => {
            return Err(unexpected_value(
                span,
                &[".f16", ".bf16"],
                format!(".{other}"),
            ));
        }
    };
    Ok((ty, span))
}

fn parse_packed_type(stream: &mut PtxTokenStream) -> Result<(PackedType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let ty = match directive.as_str() {
        "f16x2" => PackedType::F16x2,
        "bf16x2" => PackedType::Bf16x2,
        other => {
            return Err(unexpected_value(
                span,
                &[".f16x2", ".bf16x2"],
                format!(".{other}"),
            ));
        }
    };
    Ok((ty, span))
}

fn parse_vec16_registers(stream: &mut PtxTokenStream) -> Result<Vec16Registers, PtxParseError> {
    stream.expect(&PtxToken::LBrace)?;
    let mut registers = Vec::new();
    loop {
        registers.push(RegisterOperand::parse(stream)?);
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            continue;
        }
        break;
    }
    stream.expect(&PtxToken::RBrace)?;

    match registers.len() {
        2 => {
            let mut iter = registers.into_iter();
            Ok(Vec16Registers::V2([
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]))
        }
        4 => {
            let mut iter = registers.into_iter();
            Ok(Vec16Registers::V4([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]))
        }
        8 => {
            let mut iter = registers.into_iter();
            Ok(Vec16Registers::V8([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]))
        }
        other => Err(unexpected_value(
            0..0,
            &["vector of 2, 4, or 8 registers"],
            format!("{other} elements"),
        )),
    }
}

fn parse_vec32_registers(stream: &mut PtxTokenStream) -> Result<Vec32Registers, PtxParseError> {
    stream.expect(&PtxToken::LBrace)?;
    let mut registers = Vec::new();
    loop {
        registers.push(RegisterOperand::parse(stream)?);
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            continue;
        }
        break;
    }
    stream.expect(&PtxToken::RBrace)?;

    match registers.len() {
        2 => {
            let mut iter = registers.into_iter();
            Ok(Vec32Registers::V2([
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]))
        }
        4 => {
            let mut iter = registers.into_iter();
            Ok(Vec32Registers::V4([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]))
        }
        other => Err(unexpected_value(
            0..0,
            &["vector of 2 or 4 registers"],
            format!("{other} elements"),
        )),
    }
}

fn ensure_no_secondary_state_space(
    state_space: &Option<(StateSpace, Span)>,
) -> Result<(), PtxParseError> {
    if let Some((StateSpace::Shared(_), span)) = state_space.as_ref() {
        return Err(unexpected_value(
            span.clone(),
            &[".global"],
            ".shared".to_string(),
        ));
    }
    Ok(())
}

fn convert_state_space(state_space: Option<(StateSpace, Span)>) -> Option<StateSpace> {
    state_space.map(|(space, _)| space)
}

fn convert_vector_state_space(
    state_space: Option<(StateSpace, Span)>,
) -> Result<Option<VectorStateSpace>, PtxParseError> {
    match state_space {
        None => Ok(None),
        Some((StateSpace::Global, _)) => Ok(Some(VectorStateSpace::Global)),
        Some((StateSpace::Shared(_), span)) => {
            Err(unexpected_value(span, &[".global"], ".shared".to_string()))
        }
    }
}

fn parse_scalar_operands(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        RegisterOperand,
        AddressOperand,
        RegisterOperand,
        Option<RegisterOperand>,
    ),
    PtxParseError,
> {
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = RegisterOperand::parse(stream)?;

    let cache_policy = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(RegisterOperand::parse(stream)?)
    } else {
        None
    };

    stream.expect(&PtxToken::Semicolon)?;
    Ok((destination, address, source, cache_policy))
}

fn parse_compare_swap_operands(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        RegisterOperand,
        AddressOperand,
        RegisterOperand,
        RegisterOperand,
    ),
    PtxParseError,
> {
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let compare = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let new_value = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok((destination, address, compare, new_value))
}

fn parse_vector_operands32(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        Vec32Registers,
        AddressOperand,
        Vec32Registers,
        Option<RegisterOperand>,
    ),
    PtxParseError,
> {
    let destination = parse_vec32_registers(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = parse_vec32_registers(stream)?;

    let cache_policy = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(RegisterOperand::parse(stream)?)
    } else {
        None
    };

    stream.expect(&PtxToken::Semicolon)?;
    Ok((destination, address, source, cache_policy))
}

fn parse_vector_operands16(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        Vec16Registers,
        AddressOperand,
        Vec16Registers,
        Option<RegisterOperand>,
    ),
    PtxParseError,
> {
    let destination = parse_vec16_registers(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = parse_vec16_registers(stream)?;

    let cache_policy = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(RegisterOperand::parse(stream)?)
    } else {
        None
    };

    stream.expect(&PtxToken::Semicolon)?;
    Ok((destination, address, source, cache_policy))
}

fn parse_compare_swap_variant(
    stream: &mut PtxTokenStream,
) -> Result<(CompareSwapVariant, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    match directive.as_str() {
        "b16" => Ok((CompareSwapVariant::B16, span)),
        "b128" => Ok((CompareSwapVariant::B128, span)),
        "b32" | "b64" | "u32" | "u64" | "s32" | "s64" | "f32" | "f64" => {
            let data_span = span.clone();
            let data_type = match directive.as_str() {
                "b32" => DataType::B32,
                "b64" => DataType::B64,
                "u32" => DataType::U32,
                "u64" => DataType::U64,
                "s32" => DataType::S32,
                "s64" => DataType::S64,
                "f32" => DataType::F32,
                "f64" => DataType::F64,
                _ => unreachable!(),
            };
            Ok((CompareSwapVariant::Typed(data_type), data_span))
        }
        other => Err(unexpected_value(
            span,
            &[
                ".b16", ".b128", ".b32", ".b64", ".u32", ".u64", ".s32", ".s64", ".f32", ".f64",
            ],
            format!(".{other}"),
        )),
    }
}

fn parse_scalar_instruction(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    operation: ScalarOperation,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<Atom, PtxParseError> {
    let (data_type, _) = parse_data_type(stream)?;
    let (destination, address, source, cache_policy) = parse_scalar_operands(stream)?;
    Ok(Atom::Scalar(Scalar {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: convert_state_space(state_space),
        operation,
        cache_hint: cache_hint.map(|(value, _)| value),
        data_type,
        destination,
        address,
        source,
        cache_policy,
    }))
}

fn parse_exchange128(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<Atom, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    if modifier != "b128" {
        return Err(unexpected_value(span, &[".b128"], format!(".{modifier}")));
    }

    let (destination, address, source, cache_policy) = parse_scalar_operands(stream)?;
    Ok(Atom::Exchange128(Exchange128 {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: convert_state_space(state_space),
        cache_hint: cache_hint.map(|(value, _)| value),
        destination,
        address,
        source,
        cache_policy,
    }))
}

fn parse_add_noftz(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<Atom, PtxParseError> {
    let (data_type, _) = parse_noftz_type(stream)?;
    let (destination, address, source, cache_policy) = parse_scalar_operands(stream)?;
    Ok(Atom::AddNoFtz(AddNoFtz {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: convert_state_space(state_space),
        cache_hint: cache_hint.map(|(value, _)| value),
        data_type,
        destination,
        address,
        source,
        cache_policy,
    }))
}

fn parse_vector_add32(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<Atom, PtxParseError> {
    let vector_state_space = convert_vector_state_space(state_space)?;

    let (modifier, span) = stream.expect_directive()?;
    if modifier != "vec_32_bit" {
        return Err(unexpected_value(
            span,
            &[".vec_32_bit"],
            format!(".{modifier}"),
        ));
    }

    let (data_modifier, data_span) = stream.expect_directive()?;
    if data_modifier != "f32" {
        return Err(unexpected_value(
            data_span,
            &[".f32"],
            format!(".{data_modifier}"),
        ));
    }

    let (destination, address, source, cache_policy) = parse_vector_operands32(stream)?;
    Ok(Atom::VectorAdd32(VectorAdd32 {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        destination,
        address,
        source,
        cache_policy,
    }))
}

fn parse_vector_half(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
    operation: VectorOperation,
) -> Result<Atom, PtxParseError> {
    let vector_state_space = convert_vector_state_space(state_space)?;

    let (modifier, span) = stream.expect_directive()?;
    if modifier != "vec_16_bit" {
        return Err(unexpected_value(
            span,
            &[".vec_16_bit"],
            format!(".{modifier}"),
        ));
    }

    let (element_type, _) = parse_half_word_type(stream)?;
    let (destination, address, source, cache_policy) = parse_vector_operands16(stream)?;

    Ok(Atom::VectorHalf(VectorHalf {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        operation,
        element_type,
        destination,
        address,
        source,
        cache_policy,
    }))
}

fn parse_vector_packed(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
    operation: VectorOperation,
) -> Result<Atom, PtxParseError> {
    let vector_state_space = convert_vector_state_space(state_space)?;

    let (modifier, span) = stream.expect_directive()?;
    if modifier != "vec_32_bit" {
        return Err(unexpected_value(
            span,
            &[".vec_32_bit"],
            format!(".{modifier}"),
        ));
    }

    let (element_type, _) = parse_packed_type(stream)?;
    let (destination, address, source, cache_policy) = parse_vector_operands32(stream)?;

    Ok(Atom::VectorPacked(VectorPacked {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        operation,
        element_type,
        destination,
        address,
        source,
        cache_policy,
    }))
}

impl PtxParser for Atom {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "atom" {
            return Err(unexpected_value(span, &["atom"], opcode));
        }

        let mut semantics: Option<(Semantics, Span)> = None;
        let mut scope: Option<(Scope, Span)> = None;
        let mut state_space: Option<(StateSpace, Span)> = None;

        loop {
            let Some((modifier, span)) = peek_directive(stream)? else {
                break;
            };

            match modifier.as_str() {
                "relaxed" | "acquire" | "release" | "acq_rel" => {
                    if semantics.is_some() {
                        return Err(unexpected_value(
                            span.clone(),
                            &["single semantics modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    semantics = Some(parse_semantics(stream)?);
                }
                "cta" | "cluster" | "gpu" | "sys" => {
                    if scope.is_some() {
                        return Err(unexpected_value(
                            span.clone(),
                            &["single scope modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    scope = Some(parse_scope(stream)?);
                }
                "global" | "shared" => {
                    if state_space.is_some() {
                        return Err(unexpected_value(
                            span.clone(),
                            &["single state space modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    state_space = Some(parse_state_space(stream)?);
                }
                _ => break,
            }
        }

        let (operation_token, operation_span) = stream.expect_directive()?;
        match operation_token.as_str() {
            "cas" => {
                let (variant, _) = parse_compare_swap_variant(stream)?;
                let (destination, address, compare, new_value) =
                    parse_compare_swap_operands(stream)?;
                Ok(Atom::CompareSwap(CompareSwap {
                    semantics: semantics.map(|(value, _)| value),
                    scope: scope.map(|(value, _)| value),
                    state_space: convert_state_space(state_space),
                    variant,
                    destination,
                    address,
                    compare,
                    new_value,
                }))
            }
            "exch" => {
                let cache_hint = parse_cache_hint(stream)?;
                if let Some((next, _)) = peek_directive(stream)? {
                    if next == "b128" {
                        return parse_exchange128(
                            stream,
                            semantics,
                            scope,
                            state_space,
                            cache_hint,
                        );
                    }
                }
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Exch,
                    cache_hint,
                )
            }
            "and" => {
                let cache_hint = parse_cache_hint(stream)?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::And,
                    cache_hint,
                )
            }
            "or" => {
                let cache_hint = parse_cache_hint(stream)?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Or,
                    cache_hint,
                )
            }
            "xor" => {
                let cache_hint = parse_cache_hint(stream)?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Xor,
                    cache_hint,
                )
            }
            "add" => {
                let mut cache_hint = parse_cache_hint(stream)?;
                if let Some((next, _)) = peek_directive(stream)? {
                    if next == "noftz" {
                        let (directive, _) = stream.expect_directive()?;
                        debug_assert_eq!(directive, "noftz");
                        cache_hint = parse_cache_hint(stream)?;
                        if let Some((after_noftz, _)) = peek_directive(stream)? {
                            match after_noftz.as_str() {
                                "vec_16_bit" => {
                                    let operation =
                                        parse_vector_operation("add", operation_span.clone())?;
                                    return parse_vector_half(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                        operation,
                                    );
                                }
                                "vec_32_bit" => {
                                    let operation =
                                        parse_vector_operation("add", operation_span.clone())?;
                                    return parse_vector_packed(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                        operation,
                                    );
                                }
                                _ => {
                                    return parse_add_noftz(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                    );
                                }
                            }
                        } else {
                            return parse_add_noftz(
                                stream,
                                semantics,
                                scope,
                                state_space,
                                cache_hint,
                            );
                        }
                    } else if next == "vec_32_bit" {
                        ensure_no_secondary_state_space(&state_space)?;
                        return parse_vector_add32(
                            stream,
                            semantics,
                            scope,
                            state_space,
                            cache_hint,
                        );
                    }
                }

                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Add,
                    cache_hint,
                )
            }
            "inc" => {
                let cache_hint = parse_cache_hint(stream)?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Inc,
                    cache_hint,
                )
            }
            "dec" => {
                let cache_hint = parse_cache_hint(stream)?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    ScalarOperation::Dec,
                    cache_hint,
                )
            }
            "min" | "max" => {
                let mut cache_hint = parse_cache_hint(stream)?;
                if let Some((next, _)) = peek_directive(stream)? {
                    if next == "noftz" {
                        let (directive, _) = stream.expect_directive()?;
                        debug_assert_eq!(directive, "noftz");
                        cache_hint = parse_cache_hint(stream)?;
                        let operation = parse_vector_operation(
                            operation_token.as_str(),
                            operation_span.clone(),
                        )?;
                        if let Some((after_noftz, _)) = peek_directive(stream)? {
                            match after_noftz.as_str() {
                                "vec_16_bit" => {
                                    return parse_vector_half(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                        operation,
                                    );
                                }
                                "vec_32_bit" => {
                                    return parse_vector_packed(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                        operation,
                                    );
                                }
                                _ => {}
                            }
                        }
                        return Err(unexpected_value(
                            operation_span,
                            &[".vec_16_bit", ".vec_32_bit"],
                            format!(".{}", operation_token),
                        ));
                    }
                }

                let scalar_operation = match operation_token.as_str() {
                    "min" => ScalarOperation::Min,
                    "max" => ScalarOperation::Max,
                    _ => unreachable!(),
                };

                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    scalar_operation,
                    cache_hint,
                )
            }
            other => Err(unexpected_value(
                operation_span,
                &[
                    ".cas", ".exch", ".and", ".or", ".xor", ".add", ".inc", ".dec", ".min", ".max",
                ],
                format!(".{other}"),
            )),
        }
    }
}
