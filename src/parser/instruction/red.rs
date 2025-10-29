use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::red::*},
};

fn parse_semantics(stream: &mut PtxTokenStream) -> Result<(Semantics, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let semantics = match directive.as_str() {
        "relaxed" => Semantics::Relaxed,
        "release" => Semantics::Release,
        other => {
            return Err(unexpected_value(
                span,
                &[".relaxed", ".release"],
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
            if stream.check(|token| matches!(token, PtxToken::Colon)) {
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
            } else {
                StateSpace::Shared(SharedSpace::Default)
            }
        }
        other => {
            return Err(unexpected_value(
                span,
                &[".global", ".shared", ".shared::cta", ".shared::cluster"],
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

fn parse_scalar_operation(name: &str, span: Span) -> Result<ScalarOperation, PtxParseError> {
    match name {
        "and" => Ok(ScalarOperation::And),
        "or" => Ok(ScalarOperation::Or),
        "xor" => Ok(ScalarOperation::Xor),
        "add" => Ok(ScalarOperation::Add),
        "inc" => Ok(ScalarOperation::Inc),
        "dec" => Ok(ScalarOperation::Dec),
        "min" => Ok(ScalarOperation::Min),
        "max" => Ok(ScalarOperation::Max),
        other => Err(unexpected_value(
            span,
            &[
                ".and", ".or", ".xor", ".add", ".inc", ".dec", ".min", ".max",
            ],
            format!(".{other}"),
        )),
    }
}

fn parse_scalar_type(stream: &mut PtxTokenStream) -> Result<(ScalarType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let scalar_type = match directive.as_str() {
        "b32" => ScalarType::B32,
        "b64" => ScalarType::B64,
        "u32" => ScalarType::U32,
        "u64" => ScalarType::U64,
        "s32" => ScalarType::S32,
        "s64" => ScalarType::S64,
        "f32" => ScalarType::F32,
        "f64" => ScalarType::F64,
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
    Ok((scalar_type, span))
}

fn parse_scalar_add_noftz_type(
    stream: &mut PtxTokenStream,
) -> Result<(ScalarAddNoFtzType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let ty = match directive.as_str() {
        "f16" => ScalarAddNoFtzType::F16,
        "f16x2" => ScalarAddNoFtzType::F16x2,
        "bf16" => ScalarAddNoFtzType::Bf16,
        "bf16x2" => ScalarAddNoFtzType::Bf16x2,
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

fn parse_vec32_element_type(
    stream: &mut PtxTokenStream,
) -> Result<(Vec32ElementType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let ty = match directive.as_str() {
        "f32" => Vec32ElementType::F32,
        other => return Err(unexpected_value(span, &[".f32"], format!(".{other}"))),
    };
    Ok((ty, span))
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

fn parse_scalar_operands(
    stream: &mut PtxTokenStream,
) -> Result<(AddressOperand, RegisterOperand, Option<RegisterOperand>), PtxParseError> {
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
    Ok((address, source, cache_policy))
}

fn parse_vector_operands16(
    stream: &mut PtxTokenStream,
) -> Result<(AddressOperand, Vec16Registers, Option<RegisterOperand>), PtxParseError> {
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
    Ok((address, source, cache_policy))
}

fn parse_vector_operands32(
    stream: &mut PtxTokenStream,
) -> Result<(AddressOperand, Vec32Registers, Option<RegisterOperand>), PtxParseError> {
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
    Ok((address, source, cache_policy))
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

fn parse_scalar_instruction(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    operation: ScalarOperation,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<RedOpcode, PtxParseError> {
    let (data_type, _) = parse_scalar_type(stream)?;
    let (address, source, cache_policy) = parse_scalar_operands(stream)?;

    Ok(RedOpcode::Scalar(Scalar {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: convert_state_space(state_space),
        operation,
        cache_hint: cache_hint.map(|(value, _)| value),
        data_type,
        address,
        source,
        cache_policy,
    }))
}

fn parse_scalar_add_noftz(
    stream: &mut PtxTokenStream,
    semantics: Option<(Semantics, Span)>,
    scope: Option<(Scope, Span)>,
    state_space: Option<(StateSpace, Span)>,
    cache_hint: Option<(CacheHint, Span)>,
) -> Result<RedOpcode, PtxParseError> {
    let (data_type, _) = parse_scalar_add_noftz_type(stream)?;
    let (address, source, cache_policy) = parse_scalar_operands(stream)?;

    Ok(RedOpcode::ScalarAddNoFtz(ScalarAddNoFtz {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: convert_state_space(state_space),
        cache_hint: cache_hint.map(|(value, _)| value),
        data_type,
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
) -> Result<RedOpcode, PtxParseError> {
    let vector_state_space = convert_vector_state_space(state_space)?;

    let (modifier, span) = stream.expect_directive()?;
    if modifier != "vec_32_bit" {
        return Err(unexpected_value(
            span,
            &[".vec_32_bit"],
            format!(".{modifier}"),
        ));
    }

    let (element_type, _) = parse_vec32_element_type(stream)?;
    let (address, source, cache_policy) = parse_vector_operands32(stream)?;

    Ok(RedOpcode::VectorAdd32(VectorAdd32 {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        address,
        element_type,
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
) -> Result<RedOpcode, PtxParseError> {
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
    let (address, source, cache_policy) = parse_vector_operands16(stream)?;

    Ok(RedOpcode::VectorHalf(VectorHalf {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        operation,
        element_type,
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
) -> Result<RedOpcode, PtxParseError> {
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
    let (address, source, cache_policy) = parse_vector_operands32(stream)?;

    Ok(RedOpcode::VectorPacked(VectorPacked {
        semantics: semantics.map(|(value, _)| value),
        scope: scope.map(|(value, _)| value),
        state_space: vector_state_space,
        cache_hint: cache_hint.map(|(value, _)| value),
        operation,
        element_type,
        address,
        source,
        cache_policy,
    }))
}

impl PtxParser for RedOpcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "red" {
            return Err(unexpected_value(span, &["red"], opcode));
        }

        let mut semantics: Option<(Semantics, Span)> = None;
        let mut scope: Option<(Scope, Span)> = None;
        let mut state_space: Option<(StateSpace, Span)> = None;

        loop {
            let Some((modifier, modifier_span)) = peek_directive(stream)? else {
                break;
            };

            match modifier.as_str() {
                "relaxed" | "release" => {
                    if semantics.is_some() {
                        return Err(unexpected_value(
                            modifier_span.clone(),
                            &["single semantics modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    semantics = Some(parse_semantics(stream)?);
                }
                "cta" | "cluster" | "gpu" | "sys" => {
                    if scope.is_some() {
                        return Err(unexpected_value(
                            modifier_span.clone(),
                            &["single scope modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    scope = Some(parse_scope(stream)?);
                }
                "global" | "shared" => {
                    if state_space.is_some() {
                        return Err(unexpected_value(
                            modifier_span.clone(),
                            &["single state space modifier"],
                            format!(".{modifier}"),
                        ));
                    }
                    state_space = Some(parse_state_space(stream)?);
                }
                _ => break,
            }
        }

        let (operation_name, operation_span) = stream.expect_directive()?;
        match operation_name.as_str() {
            "add" => {
                let mut cache_hint = parse_cache_hint(stream)?;
                if let Some((next, _)) = peek_directive(stream)? {
                    if next == "noftz" {
                        stream.expect_directive()?;
                        cache_hint = parse_cache_hint(stream)?;
                        if let Some((after_noftz, _)) = peek_directive(stream)? {
                            match after_noftz.as_str() {
                                "vec_16_bit" => {
                                    let operation = parse_vector_operation(
                                        operation_name.as_str(),
                                        operation_span.clone(),
                                    )?;
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
                                    let operation = parse_vector_operation(
                                        operation_name.as_str(),
                                        operation_span.clone(),
                                    )?;
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
                                    return parse_scalar_add_noftz(
                                        stream,
                                        semantics,
                                        scope,
                                        state_space,
                                        cache_hint,
                                    );
                                }
                            }
                        } else {
                            return parse_scalar_add_noftz(
                                stream,
                                semantics,
                                scope,
                                state_space,
                                cache_hint,
                            );
                        }
                    } else if next == "vec_32_bit" {
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
            "min" | "max" => {
                let mut cache_hint = parse_cache_hint(stream)?;
                if let Some((next, _)) = peek_directive(stream)? {
                    if next == "noftz" {
                        stream.expect_directive()?;
                        cache_hint = parse_cache_hint(stream)?;
                        let operation = parse_vector_operation(
                            operation_name.as_str(),
                            operation_span.clone(),
                        )?;

                        if let Some((after_noftz, span)) = peek_directive(stream)? {
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
                                other => {
                                    return Err(unexpected_value(
                                        span,
                                        &[".vec_16_bit", ".vec_32_bit"],
                                        format!(".{other}"),
                                    ));
                                }
                            }
                        } else {
                            return Err(unexpected_value(
                                operation_span.clone(),
                                &[".vec_16_bit", ".vec_32_bit"],
                                "<end>".to_string(),
                            ));
                        }
                    }
                }

                let scalar_operation =
                    parse_scalar_operation(operation_name.as_str(), operation_span.clone())?;
                parse_scalar_instruction(
                    stream,
                    semantics,
                    scope,
                    state_space,
                    scalar_operation,
                    cache_hint,
                )
            }
            "and" | "or" | "xor" | "inc" | "dec" => {
                let cache_hint = parse_cache_hint(stream)?;
                let scalar_operation =
                    parse_scalar_operation(operation_name.as_str(), operation_span)?;
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
                    ".and", ".or", ".xor", ".add", ".inc", ".dec", ".min", ".max",
                ],
                format!(".{other}"),
            )),
        }
    }
}
