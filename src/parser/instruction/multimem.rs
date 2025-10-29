use crate::parser::common::parse_register_name;
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::multimem::*},
};
use std::convert::TryInto;

const LD_REDUCE_OPERATION_NAMES: &[(&str, &str)] = &[
    ("min", ".min"),
    ("max", ".max"),
    ("add", ".add"),
    ("and", ".and"),
    ("or", ".or"),
    ("xor", ".xor"),
];

const STORE_INTEGER_TYPES: &[(&str, IntegerType, &str)] = &[
    ("b32", IntegerType::B32, ".b32"),
    ("b64", IntegerType::B64, ".b64"),
    ("u32", IntegerType::U32, ".u32"),
    ("u64", IntegerType::U64, ".u64"),
    ("s32", IntegerType::S32, ".s32"),
    ("s64", IntegerType::S64, ".s64"),
];

const STORE_FLOAT_TYPES: &[(&str, FloatType, &str)] = &[
    ("f16", FloatType::F16, ".f16"),
    ("f16x2", FloatType::F16x2, ".f16x2"),
    ("bf16", FloatType::Bf16, ".bf16"),
    ("bf16x2", FloatType::Bf16x2, ".bf16x2"),
    ("f32", FloatType::F32, ".f32"),
    ("f64", FloatType::F64, ".f64"),
    ("e5m2", FloatType::E5m2, ".e5m2"),
    ("e5m2x2", FloatType::E5m2x2, ".e5m2x2"),
    ("e5m2x4", FloatType::E5m2x4, ".e5m2x4"),
    ("e4m3", FloatType::E4m3, ".e4m3"),
    ("e4m3x2", FloatType::E4m3x2, ".e4m3x2"),
    ("e4m3x4", FloatType::E4m3x4, ".e4m3x4"),
];

const RED_FLOAT_TYPES: &[(&str, FloatReductionType, &str)] = &[
    ("f16", FloatReductionType::F16, ".f16"),
    ("f16x2", FloatReductionType::F16x2, ".f16x2"),
    ("bf16", FloatReductionType::Bf16, ".bf16"),
    ("bf16x2", FloatReductionType::Bf16x2, ".bf16x2"),
    ("f32", FloatReductionType::F32, ".f32"),
    ("f64", FloatReductionType::F64, ".f64"),
];

enum NumericType {
    Integer(IntegerType),
    Float(FloatType),
}

enum ReductionType {
    Integer(IntegerType),
    Float(FloatReductionType),
}

fn parse_optional_directive<T, F>(
    stream: &mut PtxTokenStream,
    mapper: F,
) -> Result<Option<T>, PtxParseError>
where
    F: Fn(&str) -> Option<T>,
{
    if let Some((value, _)) = peek_directive(stream)? {
        if let Some(mapped) = mapper(&value) {
            stream.consume()?;
            return Ok(Some(mapped));
        }
    }
    Ok(None)
}

fn parse_optional_load_semantics(
    stream: &mut PtxTokenStream,
) -> Result<Option<LoadSemantics>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "relaxed" => Some(LoadSemantics::Relaxed),
        "acquire" => Some(LoadSemantics::Acquire),
        _ => None,
    })
}

fn parse_optional_store_semantics(
    stream: &mut PtxTokenStream,
) -> Result<Option<StoreSemantics>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "relaxed" => Some(StoreSemantics::Relaxed),
        "release" => Some(StoreSemantics::Release),
        _ => None,
    })
}

fn parse_optional_reduction_semantics(
    stream: &mut PtxTokenStream,
) -> Result<Option<ReductionSemantics>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "relaxed" => Some(ReductionSemantics::Relaxed),
        "release" => Some(ReductionSemantics::Release),
        _ => None,
    })
}

fn parse_optional_scope(stream: &mut PtxTokenStream) -> Result<Option<Scope>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "cta" => Some(Scope::Cta),
        "cluster" => Some(Scope::Cluster),
        "gpu" => Some(Scope::Gpu),
        "sys" => Some(Scope::Sys),
        _ => None,
    })
}

fn parse_optional_state_space(
    stream: &mut PtxTokenStream,
) -> Result<Option<StateSpace>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "global" => Some(StateSpace::Global),
        _ => None,
    })
}

fn parse_optional_accumulator_precision(
    stream: &mut PtxTokenStream,
) -> Result<Option<AccumulatorPrecision>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "acc_f32" => Some(AccumulatorPrecision::AccF32),
        "acc_f16" => Some(AccumulatorPrecision::AccF16),
        _ => None,
    })
}

fn parse_optional_vector_width(
    stream: &mut PtxTokenStream,
) -> Result<Option<VectorWidth>, PtxParseError> {
    parse_optional_directive(stream, |value| match value {
        "v2" => Some(VectorWidth::V2),
        "v4" => Some(VectorWidth::V4),
        "v8" => Some(VectorWidth::V8),
        _ => None,
    })
}

fn parse_operation_name(
    stream: &mut PtxTokenStream,
    allowed: &[(&str, &str)],
) -> Result<(String, Span), PtxParseError> {
    let (name, span) = stream.expect_directive()?;
    if allowed.iter().any(|(raw, _)| *raw == name.as_str()) {
        Ok((name, span))
    } else {
        let expected: Vec<&str> = allowed.iter().map(|(_, display)| *display).collect();
        Err(unexpected_value(span, &expected, format!(".{name}")))
    }
}

fn numeric_type_from_directive(value: &str, span: Span) -> Result<NumericType, PtxParseError> {
    if let Some((_, ty, _)) = STORE_INTEGER_TYPES
        .iter()
        .find(|(name, _, _)| *name == value)
    {
        return Ok(NumericType::Integer(*ty));
    }

    if let Some((_, ty, _)) = STORE_FLOAT_TYPES.iter().find(|(name, _, _)| *name == value) {
        return Ok(NumericType::Float(*ty));
    }

    let mut expected: Vec<&str> = STORE_INTEGER_TYPES
        .iter()
        .map(|(_, _, display)| *display)
        .collect();
    expected.extend(STORE_FLOAT_TYPES.iter().map(|(_, _, display)| *display));
    Err(unexpected_value(span, &expected, format!(".{value}")))
}

fn reduction_type_from_directive(value: &str, span: Span) -> Result<ReductionType, PtxParseError> {
    if let Some((_, ty, _)) = STORE_INTEGER_TYPES
        .iter()
        .find(|(name, _, _)| *name == value)
    {
        return Ok(ReductionType::Integer(*ty));
    }

    if let Some((_, ty, _)) = RED_FLOAT_TYPES.iter().find(|(name, _, _)| *name == value) {
        return Ok(ReductionType::Float(*ty));
    }

    let mut expected: Vec<&str> = STORE_INTEGER_TYPES
        .iter()
        .map(|(_, _, display)| *display)
        .collect();
    expected.extend(RED_FLOAT_TYPES.iter().map(|(_, _, display)| *display));
    Err(unexpected_value(span, &expected, format!(".{value}")))
}

fn parse_scalar_register(stream: &mut PtxTokenStream) -> Result<RegisterOperand, PtxParseError> {
    let (name, _) = parse_register_name(stream)?;
    Ok(RegisterOperand::Single(name))
}

fn parse_vector_operands<const N: usize>(
    stream: &mut PtxTokenStream,
) -> Result<[RegisterOperand; N], PtxParseError> {
    let (_, span) = stream.expect(&PtxToken::LBrace)?;
    let mut registers = Vec::with_capacity(N);
    for index in 0..N {
        let (name, _) = parse_register_name(stream)?;
        registers.push(RegisterOperand::Single(name));
        if index < N - 1 {
            stream.expect(&PtxToken::Comma)?;
        }
    }
    stream.expect(&PtxToken::RBrace)?;
    let array: [RegisterOperand; N] = registers
        .try_into()
        .map_err(|_| invalid_literal(span.clone(), "invalid register vector length"))?;
    Ok(array)
}

fn parse_vector_destination(
    stream: &mut PtxTokenStream,
    width: Option<VectorWidth>,
) -> Result<VectorDestination, PtxParseError> {
    match width {
        Some(VectorWidth::V2) => Ok(VectorDestination::Vector2(parse_vector_operands::<2>(
            stream,
        )?)),
        Some(VectorWidth::V4) => Ok(VectorDestination::Vector4(parse_vector_operands::<4>(
            stream,
        )?)),
        Some(VectorWidth::V8) => Ok(VectorDestination::Vector8(parse_vector_operands::<8>(
            stream,
        )?)),
        None => Ok(VectorDestination::Scalar(parse_scalar_register(stream)?)),
    }
}

fn parse_vector_value(
    stream: &mut PtxTokenStream,
    width: Option<VectorWidth>,
) -> Result<VectorValue, PtxParseError> {
    match width {
        Some(VectorWidth::V2) => Ok(VectorValue::Vector2(parse_vector_operands::<2>(stream)?)),
        Some(VectorWidth::V4) => Ok(VectorValue::Vector4(parse_vector_operands::<4>(stream)?)),
        Some(VectorWidth::V8) => Ok(VectorValue::Vector8(parse_vector_operands::<8>(stream)?)),
        None => Ok(VectorValue::Scalar(parse_scalar_register(stream)?)),
    }
}

fn map_integer_op(name: &str) -> Option<IntegerOp> {
    match name {
        "min" => Some(IntegerOp::Min),
        "max" => Some(IntegerOp::Max),
        "add" => Some(IntegerOp::Add),
        "and" => Some(IntegerOp::And),
        "or" => Some(IntegerOp::Or),
        "xor" => Some(IntegerOp::Xor),
        _ => None,
    }
}

fn map_float_op(name: &str) -> Option<FloatOp> {
    match name {
        "min" => Some(FloatOp::Min),
        "max" => Some(FloatOp::Max),
        "add" => Some(FloatOp::Add),
        _ => None,
    }
}

fn map_float_red_op(name: &str) -> Option<FloatRedOp> {
    match name {
        "add" => Some(FloatRedOp::Add),
        _ => None,
    }
}

fn expect_integer_operation(name: &str, span: Span) -> Result<IntegerOp, PtxParseError> {
    map_integer_op(name).ok_or_else(|| {
        let expected = vec![".min", ".max", ".add", ".and", ".or", ".xor"];
        unexpected_value(span, &expected, format!(".{name}"))
    })
}

fn expect_float_operation(name: &str, span: Span) -> Result<FloatOp, PtxParseError> {
    map_float_op(name).ok_or_else(|| {
        let expected = vec![".min", ".max", ".add"];
        unexpected_value(span, &expected, format!(".{name}"))
    })
}

fn expect_float_red_operation(name: &str, span: Span) -> Result<FloatRedOp, PtxParseError> {
    map_float_red_op(name).ok_or_else(|| {
        let expected = vec![".add"];
        unexpected_value(span, &expected, format!(".{name}"))
    })
}

fn parse_ld_reduce(stream: &mut PtxTokenStream, weak: bool) -> Result<LdReduce, PtxParseError> {
    let semantics = if weak {
        None
    } else {
        parse_optional_load_semantics(stream)?
    };
    let scope = if weak {
        None
    } else {
        parse_optional_scope(stream)?
    };
    let state_space = parse_optional_state_space(stream)?;
    let (operation_name, operation_span) = parse_operation_name(stream, LD_REDUCE_OPERATION_NAMES)?;
    let accumulator_precision = parse_optional_accumulator_precision(stream)?;
    let vector = parse_optional_vector_width(stream)?;
    let (data_type_name, data_type_span) = stream.expect_directive()?;
    let data_type = numeric_type_from_directive(&data_type_name, data_type_span.clone())?;

    match data_type {
        NumericType::Integer(data_type) => {
            if accumulator_precision.is_some() {
                return Err(invalid_literal(
                    data_type_span,
                    "accumulator precision is only supported for floating-point variants",
                ));
            }
            if vector.is_some() {
                return Err(invalid_literal(
                    data_type_span,
                    "vector widths are only supported for floating-point variants",
                ));
            }

            let destination = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let address = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            let operation = expect_integer_operation(&operation_name, operation_span)?;

            if weak {
                Ok(LdReduce::WeakInt(LdReduceWeakInt {
                    state_space,
                    operation,
                    data_type,
                    destination,
                    address,
                }))
            } else {
                Ok(LdReduce::Int(LdReduceInt {
                    semantics,
                    scope,
                    state_space,
                    operation,
                    data_type,
                    destination,
                    address,
                }))
            }
        }
        NumericType::Float(data_type) => {
            let destination = parse_vector_destination(stream, vector)?;
            stream.expect(&PtxToken::Comma)?;
            let address = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            let operation = expect_float_operation(&operation_name, operation_span)?;

            if weak {
                Ok(LdReduce::WeakFloat(LdReduceWeakFloat {
                    state_space,
                    operation,
                    accumulator_precision,
                    vector,
                    data_type,
                    destination,
                    address,
                }))
            } else {
                Ok(LdReduce::Float(LdReduceFloat {
                    semantics,
                    scope,
                    state_space,
                    operation,
                    accumulator_precision,
                    vector,
                    data_type,
                    destination,
                    address,
                }))
            }
        }
    }
}

fn parse_store(stream: &mut PtxTokenStream, weak: bool) -> Result<Store, PtxParseError> {
    let semantics = if weak {
        None
    } else {
        parse_optional_store_semantics(stream)?
    };
    let scope = if weak {
        None
    } else {
        parse_optional_scope(stream)?
    };
    let state_space = parse_optional_state_space(stream)?;
    let vector = parse_optional_vector_width(stream)?;
    let (data_type_name, data_type_span) = stream.expect_directive()?;
    let data_type = numeric_type_from_directive(&data_type_name, data_type_span.clone())?;

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    match data_type {
        NumericType::Integer(data_type) => {
            if vector.is_some() {
                return Err(invalid_literal(
                    data_type_span,
                    "vector widths are only supported for floating-point variants",
                ));
            }

            let value = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            if weak {
                Ok(Store::WeakInt(StoreWeakInt {
                    state_space,
                    data_type,
                    address,
                    value,
                }))
            } else {
                Ok(Store::Int(StoreInt {
                    semantics,
                    scope,
                    state_space,
                    data_type,
                    address,
                    value,
                }))
            }
        }
        NumericType::Float(data_type) => {
            let value = parse_vector_value(stream, vector)?;
            stream.expect(&PtxToken::Semicolon)?;

            if weak {
                Ok(Store::WeakFloat(StoreWeakFloat {
                    state_space,
                    vector,
                    data_type,
                    address,
                    value,
                }))
            } else {
                Ok(Store::Float(StoreFloat {
                    semantics,
                    scope,
                    state_space,
                    vector,
                    data_type,
                    address,
                    value,
                }))
            }
        }
    }
}

fn parse_red(stream: &mut PtxTokenStream) -> Result<Red, PtxParseError> {
    let semantics = parse_optional_reduction_semantics(stream)?;
    let scope = parse_optional_scope(stream)?;
    let state_space = parse_optional_state_space(stream)?;
    let (operation_name, operation_span) = parse_operation_name(stream, LD_REDUCE_OPERATION_NAMES)?;
    let vector = parse_optional_vector_width(stream)?;
    let (data_type_name, data_type_span) = stream.expect_directive()?;
    let data_type = reduction_type_from_directive(&data_type_name, data_type_span.clone())?;

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    match data_type {
        ReductionType::Integer(data_type) => {
            if vector.is_some() {
                return Err(invalid_literal(
                    data_type_span,
                    "vector widths are only supported for floating-point variants",
                ));
            }

            let value = RegisterOperand::parse(stream)?;
            stream.expect(&PtxToken::Semicolon)?;

            let operation = expect_integer_operation(&operation_name, operation_span)?;

            Ok(Red::Int(RedInt {
                semantics,
                scope,
                state_space,
                operation,
                data_type,
                address,
                value,
            }))
        }
        ReductionType::Float(data_type) => {
            let value = parse_vector_value(stream, vector)?;
            stream.expect(&PtxToken::Semicolon)?;

            let operation = expect_float_red_operation(&operation_name, operation_span)?;

            Ok(Red::Float(RedFloat {
                semantics,
                scope,
                state_space,
                operation,
                vector,
                data_type,
                address,
                value,
            }))
        }
    }
}

impl PtxParser for Instruction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "multimem" {
            return Err(unexpected_value(span, &["multimem"], opcode));
        }

        let (sub_opcode, sub_span) = stream.expect_directive()?;
        match sub_opcode.as_str() {
            "ld_reduce" => {
                let weak = consume_directive_if(stream, "weak");
                let instruction = parse_ld_reduce(stream, weak)?;
                Ok(Instruction::LdReduce(instruction))
            }
            "st" => {
                let weak = consume_directive_if(stream, "weak");
                let instruction = parse_store(stream, weak)?;
                Ok(Instruction::Store(instruction))
            }
            "red" => {
                let instruction = parse_red(stream)?;
                Ok(Instruction::Red(instruction))
            }
            other => Err(unexpected_value(
                sub_span,
                &[".ld_reduce", ".st", ".red"],
                format!(".{other}"),
            )),
        }
    }
}
