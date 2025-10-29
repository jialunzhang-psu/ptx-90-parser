use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::wgmma::*},
};

fn convert_from_table<T: Copy>(
    value: &str,
    span: Span,
    table: &[(&'static str, T)],
) -> Result<T, PtxParseError> {
    for (literal, variant) in table {
        if value == *literal {
            return Ok(*variant);
        }
    }

    Err(PtxParseError {
        kind: ParseErrorKind::UnexpectedToken {
            expected: table
                .iter()
                .map(|(literal, _)| format!(".{literal}"))
                .collect(),
            found: format!(".{value}"),
        },
        span,
    })
}

fn convert_shape_k16(value: &str, span: Span) -> Result<ShapeK16, PtxParseError> {
    convert_from_table(
        value,
        span,
        &[
            ("m64n8k16", ShapeK16::M64N8K16),
            ("m64n16k16", ShapeK16::M64N16K16),
            ("m64n24k16", ShapeK16::M64N24K16),
            ("m64n32k16", ShapeK16::M64N32K16),
            ("m64n40k16", ShapeK16::M64N40K16),
            ("m64n48k16", ShapeK16::M64N48K16),
            ("m64n56k16", ShapeK16::M64N56K16),
            ("m64n64k16", ShapeK16::M64N64K16),
            ("m64n72k16", ShapeK16::M64N72K16),
            ("m64n80k16", ShapeK16::M64N80K16),
            ("m64n88k16", ShapeK16::M64N88K16),
            ("m64n96k16", ShapeK16::M64N96K16),
            ("m64n104k16", ShapeK16::M64N104K16),
            ("m64n112k16", ShapeK16::M64N112K16),
            ("m64n120k16", ShapeK16::M64N120K16),
            ("m64n128k16", ShapeK16::M64N128K16),
            ("m64n136k16", ShapeK16::M64N136K16),
            ("m64n144k16", ShapeK16::M64N144K16),
            ("m64n152k16", ShapeK16::M64N152K16),
            ("m64n160k16", ShapeK16::M64N160K16),
            ("m64n168k16", ShapeK16::M64N168K16),
            ("m64n176k16", ShapeK16::M64N176K16),
            ("m64n184k16", ShapeK16::M64N184K16),
            ("m64n192k16", ShapeK16::M64N192K16),
            ("m64n200k16", ShapeK16::M64N200K16),
            ("m64n208k16", ShapeK16::M64N208K16),
            ("m64n216k16", ShapeK16::M64N216K16),
            ("m64n224k16", ShapeK16::M64N224K16),
            ("m64n232k16", ShapeK16::M64N232K16),
            ("m64n240k16", ShapeK16::M64N240K16),
            ("m64n248k16", ShapeK16::M64N248K16),
            ("m64n256k16", ShapeK16::M64N256K16),
        ],
    )
}

fn convert_shape_k8(value: &str, span: Span) -> Result<ShapeK8, PtxParseError> {
    convert_from_table(
        value,
        span,
        &[
            ("m64n8k8", ShapeK8::M64N8K8),
            ("m64n16k8", ShapeK8::M64N16K8),
            ("m64n24k8", ShapeK8::M64N24K8),
            ("m64n32k8", ShapeK8::M64N32K8),
            ("m64n40k8", ShapeK8::M64N40K8),
            ("m64n48k8", ShapeK8::M64N48K8),
            ("m64n56k8", ShapeK8::M64N56K8),
            ("m64n64k8", ShapeK8::M64N64K8),
            ("m64n72k8", ShapeK8::M64N72K8),
            ("m64n80k8", ShapeK8::M64N80K8),
            ("m64n88k8", ShapeK8::M64N88K8),
            ("m64n96k8", ShapeK8::M64N96K8),
            ("m64n104k8", ShapeK8::M64N104K8),
            ("m64n112k8", ShapeK8::M64N112K8),
            ("m64n120k8", ShapeK8::M64N120K8),
            ("m64n128k8", ShapeK8::M64N128K8),
            ("m64n136k8", ShapeK8::M64N136K8),
            ("m64n144k8", ShapeK8::M64N144K8),
            ("m64n152k8", ShapeK8::M64N152K8),
            ("m64n160k8", ShapeK8::M64N160K8),
            ("m64n168k8", ShapeK8::M64N168K8),
            ("m64n176k8", ShapeK8::M64N176K8),
            ("m64n184k8", ShapeK8::M64N184K8),
            ("m64n192k8", ShapeK8::M64N192K8),
            ("m64n200k8", ShapeK8::M64N200K8),
            ("m64n208k8", ShapeK8::M64N208K8),
            ("m64n216k8", ShapeK8::M64N216K8),
            ("m64n224k8", ShapeK8::M64N224K8),
            ("m64n232k8", ShapeK8::M64N232K8),
            ("m64n240k8", ShapeK8::M64N240K8),
            ("m64n248k8", ShapeK8::M64N248K8),
            ("m64n256k8", ShapeK8::M64N256K8),
        ],
    )
}

fn convert_shape_k32(value: &str, span: Span) -> Result<ShapeK32, PtxParseError> {
    convert_from_table(
        value,
        span,
        &[
            ("m64n8k32", ShapeK32::M64N8K32),
            ("m64n16k32", ShapeK32::M64N16K32),
            ("m64n24k32", ShapeK32::M64N24K32),
            ("m64n32k32", ShapeK32::M64N32K32),
            ("m64n40k32", ShapeK32::M64N40K32),
            ("m64n48k32", ShapeK32::M64N48K32),
            ("m64n56k32", ShapeK32::M64N56K32),
            ("m64n64k32", ShapeK32::M64N64K32),
            ("m64n72k32", ShapeK32::M64N72K32),
            ("m64n80k32", ShapeK32::M64N80K32),
            ("m64n88k32", ShapeK32::M64N88K32),
            ("m64n96k32", ShapeK32::M64N96K32),
            ("m64n104k32", ShapeK32::M64N104K32),
            ("m64n112k32", ShapeK32::M64N112K32),
            ("m64n120k32", ShapeK32::M64N120K32),
            ("m64n128k32", ShapeK32::M64N128K32),
            ("m64n136k32", ShapeK32::M64N136K32),
            ("m64n144k32", ShapeK32::M64N144K32),
            ("m64n152k32", ShapeK32::M64N152K32),
            ("m64n160k32", ShapeK32::M64N160K32),
            ("m64n168k32", ShapeK32::M64N168K32),
            ("m64n176k32", ShapeK32::M64N176K32),
            ("m64n184k32", ShapeK32::M64N184K32),
            ("m64n192k32", ShapeK32::M64N192K32),
            ("m64n200k32", ShapeK32::M64N200K32),
            ("m64n208k32", ShapeK32::M64N208K32),
            ("m64n216k32", ShapeK32::M64N216K32),
            ("m64n224k32", ShapeK32::M64N224K32),
            ("m64n232k32", ShapeK32::M64N232K32),
            ("m64n240k32", ShapeK32::M64N240K32),
            ("m64n248k32", ShapeK32::M64N248K32),
            ("m64n256k32", ShapeK32::M64N256K32),
        ],
    )
}

fn convert_integer_shape(value: &str, span: Span) -> Result<IntegerShape, PtxParseError> {
    convert_from_table(
        value,
        span,
        &[
            ("m64n8k32", IntegerShape::M64N8K32),
            ("m64n16k32", IntegerShape::M64N16K32),
            ("m64n24k32", IntegerShape::M64N24K32),
            ("m64n32k32", IntegerShape::M64N32K32),
            ("m64n48k32", IntegerShape::M64N48K32),
            ("m64n64k32", IntegerShape::M64N64K32),
            ("m64n80k32", IntegerShape::M64N80K32),
            ("m64n96k32", IntegerShape::M64N96K32),
            ("m64n112k32", IntegerShape::M64N112K32),
            ("m64n128k32", IntegerShape::M64N128K32),
            ("m64n144k32", IntegerShape::M64N144K32),
            ("m64n160k32", IntegerShape::M64N160K32),
            ("m64n176k32", IntegerShape::M64N176K32),
            ("m64n192k32", IntegerShape::M64N192K32),
            ("m64n208k32", IntegerShape::M64N208K32),
            ("m64n224k32", IntegerShape::M64N224K32),
        ],
    )
}

fn convert_bit_shape(value: &str, span: Span) -> Result<BitShape, PtxParseError> {
    convert_from_table(
        value,
        span,
        &[
            ("m64n8k256", BitShape::M64N8K256),
            ("m64n16k256", BitShape::M64N16K256),
            ("m64n24k256", BitShape::M64N24K256),
            ("m64n32k256", BitShape::M64N32K256),
            ("m64n48k256", BitShape::M64N48K256),
            ("m64n64k256", BitShape::M64N64K256),
            ("m64n80k256", BitShape::M64N80K256),
            ("m64n96k256", BitShape::M64N96K256),
            ("m64n112k256", BitShape::M64N112K256),
            ("m64n128k256", BitShape::M64N128K256),
            ("m64n144k256", BitShape::M64N144K256),
            ("m64n160k256", BitShape::M64N160K256),
            ("m64n176k256", BitShape::M64N176K256),
            ("m64n192k256", BitShape::M64N192K256),
            ("m64n208k256", BitShape::M64N208K256),
            ("m64n224k256", BitShape::M64N224K256),
            ("m64n240k256", BitShape::M64N240K256),
            ("m64n256k256", BitShape::M64N256K256),
        ],
    )
}

fn parse_fp8_input_type(stream: &mut PtxTokenStream) -> Result<Fp8InputType, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "e4m3" => Ok(Fp8InputType::E4M3),
        "e5m2" => Ok(Fp8InputType::E5M2),
        other => Err(unexpected_value(
            span,
            &[".e4m3", ".e5m2"],
            format!(".{other}"),
        )),
    }
}

fn parse_integer_input_type(
    stream: &mut PtxTokenStream,
) -> Result<IntegerInputType, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "s8" => Ok(IntegerInputType::S8),
        "u8" => Ok(IntegerInputType::U8),
        other => Err(unexpected_value(span, &[".s8", ".u8"], format!(".{other}"))),
    }
}

fn parse_bit_operation(stream: &mut PtxTokenStream) -> Result<BitOperation, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "and" => Ok(BitOperation::And),
        other => Err(unexpected_value(span, &[".and"], format!(".{other}"))),
    }
}

fn parse_scale_immediate(stream: &mut PtxTokenStream) -> Result<ScaleImmediate, PtxParseError> {
    let mut sign = 1;
    if stream
        .consume_if(|token| matches!(token, PtxToken::Plus))
        .is_some()
    {
        sign = 1;
    } else if stream
        .consume_if(|token| matches!(token, PtxToken::Minus))
        .is_some()
    {
        sign = -1;
    }

    let (token, span) = stream.consume()?;
    match token {
        PtxToken::DecimalInteger(value) if value == "1" => {
            if sign == -1 {
                Ok(ScaleImmediate::MinusOne)
            } else {
                Ok(ScaleImmediate::PlusOne)
            }
        }
        PtxToken::DecimalInteger(value) if value == "0" => Err(invalid_literal(
            span.clone(),
            "scale immediate must be +/-1",
        )),
        other => Err(unexpected_value(
            span.clone(),
            &["integer literal"],
            format!("{other:?}"),
        )),
    }
}

fn parse_transpose_immediate(
    stream: &mut PtxTokenStream,
) -> Result<TransposeImmediate, PtxParseError> {
    let (token, span) = stream.consume()?;
    match token {
        PtxToken::DecimalInteger(value) if value == "0" => Ok(TransposeImmediate::Identity),
        PtxToken::DecimalInteger(value) if value == "1" => Ok(TransposeImmediate::Transpose),
        other => Err(unexpected_value(
            span.clone(),
            &["integer literal (0 or 1)"],
            format!("{other:?}"),
        )),
    }
}

fn is_register_vector(register: &RegisterOperand) -> bool {
    matches!(
        register,
        RegisterOperand::Vector2(_) | RegisterOperand::Vector4(_)
    )
}

impl PtxParser for Wgmma {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "wgmma")?;
        expect_directive_value(stream, "mma_async")?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;
        expect_directive_value(stream, "shape")?;

        let (shape_literal, shape_span) = stream.expect_directive()?;
        let mut satfinite = false;
        let mut satfinite_span = None;

        if let Some((modifier, span)) = peek_directive(stream)? {
            if modifier == "satfinite" {
                stream.consume()?;
                satfinite = true;
                satfinite_span = Some(span);
            }
        }

        let (next_directive, directive_span) =
            peek_directive(stream)?.ok_or_else(|| PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span: shape_span.clone(),
            })?;

        match next_directive.as_str() {
            "dtype" => {
                stream.consume()?;
                let (dtype_token, dtype_span) = stream.expect_directive()?;
                let (next_after_dtype, _) =
                    peek_directive(stream)?.ok_or_else(|| PtxParseError {
                        kind: ParseErrorKind::UnexpectedEof,
                        span: dtype_span.clone(),
                    })?;

                if satfinite {
                    let span = satfinite_span.clone().unwrap();
                    return Err(unexpected_value(span, &[".s32"], ".satfinite".to_string()));
                }

                match next_after_dtype.as_str() {
                    "f16" => {
                        let dtype = match dtype_token.as_str() {
                            "f16" => HalfAccumulatorType::F16,
                            "f32" => HalfAccumulatorType::F32,
                            other => {
                                return Err(unexpected_value(
                                    dtype_span.clone(),
                                    &[".f16", ".f32"],
                                    format!(".{other}"),
                                ));
                            }
                        };
                        let shape = convert_shape_k16(&shape_literal, shape_span)?;
                        expect_directive_value(stream, "f16")?;

                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a_operand = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let b_descriptor = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let scale_d = PredicateRegister::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_a = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_b = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let first_transpose = parse_transpose_immediate(stream)?;

                        let mut second_transpose = None;
                        if stream
                            .consume_if(|token| matches!(token, PtxToken::Comma))
                            .is_some()
                        {
                            second_transpose = Some(parse_transpose_immediate(stream)?);
                        }

                        stream.expect(&PtxToken::Semicolon)?;

                        return if let Some(imm_trans_b) = second_transpose {
                            Ok(Wgmma::F16Descriptor(F16Descriptor {
                                shape,
                                dtype,
                                destination,
                                a_descriptor: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                                imm_trans_a: first_transpose,
                                imm_trans_b,
                            }))
                        } else {
                            Ok(Wgmma::F16Register(F16Register {
                                shape,
                                dtype,
                                destination,
                                a_register: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                                imm_trans_b: first_transpose,
                            }))
                        };
                    }
                    "bf16" => {
                        let dtype = match dtype_token.as_str() {
                            "bf16" | "f32" => Bf16AccumulatorType::F32,
                            other => {
                                return Err(unexpected_value(
                                    dtype_span.clone(),
                                    &[".bf16", ".f32"],
                                    format!(".{other}"),
                                ));
                            }
                        };
                        let shape = convert_shape_k16(&shape_literal, shape_span)?;
                        expect_directive_value(stream, "bf16")?;

                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a_operand = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let b_descriptor = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let scale_d = PredicateRegister::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_a = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_b = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let first_transpose = parse_transpose_immediate(stream)?;

                        let mut second_transpose = None;
                        if stream
                            .consume_if(|token| matches!(token, PtxToken::Comma))
                            .is_some()
                        {
                            second_transpose = Some(parse_transpose_immediate(stream)?);
                        }

                        stream.expect(&PtxToken::Semicolon)?;

                        return if let Some(imm_trans_b) = second_transpose {
                            Ok(Wgmma::Bf16Descriptor(Bf16Descriptor {
                                shape,
                                dtype,
                                destination,
                                a_descriptor: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                                imm_trans_a: first_transpose,
                                imm_trans_b,
                            }))
                        } else {
                            Ok(Wgmma::Bf16Register(Bf16Register {
                                shape,
                                dtype,
                                destination,
                                a_register: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                                imm_trans_b: first_transpose,
                            }))
                        };
                    }
                    "tf32" => {
                        let dtype = match dtype_token.as_str() {
                            "tf32" | "f32" => Tf32AccumulatorType::F32,
                            other => {
                                return Err(unexpected_value(
                                    dtype_span.clone(),
                                    &[".tf32", ".f32"],
                                    format!(".{other}"),
                                ));
                            }
                        };
                        let shape = convert_shape_k8(&shape_literal, shape_span)?;
                        expect_directive_value(stream, "tf32")?;

                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a_operand = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let b_descriptor = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let scale_d = PredicateRegister::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_a = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_b = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;

                        return if is_register_vector(&a_operand) {
                            Ok(Wgmma::Tf32Register(Tf32Register {
                                shape,
                                dtype,
                                destination,
                                a_register: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                            }))
                        } else {
                            Ok(Wgmma::Tf32Descriptor(Tf32Descriptor {
                                shape,
                                dtype,
                                destination,
                                a_descriptor: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                            }))
                        };
                    }
                    "atype" => {
                        let dtype = match dtype_token.as_str() {
                            "f16" => Fp8AccumulatorType::F16,
                            "f32" => Fp8AccumulatorType::F32,
                            other => {
                                return Err(unexpected_value(
                                    dtype_span.clone(),
                                    &[".f16", ".f32"],
                                    format!(".{other}"),
                                ));
                            }
                        };
                        let shape = convert_shape_k32(&shape_literal, shape_span)?;
                        expect_directive_value(stream, "atype")?;
                        let atype = parse_fp8_input_type(stream)?;
                        expect_directive_value(stream, "btype")?;
                        let btype = parse_fp8_input_type(stream)?;

                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let a_operand = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let b_descriptor = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let scale_d = PredicateRegister::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_a = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let imm_scale_b = parse_scale_immediate(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;

                        return if is_register_vector(&a_operand) {
                            Ok(Wgmma::Fp8Register(Fp8Register {
                                shape,
                                dtype,
                                atype,
                                btype,
                                destination,
                                a_register: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                            }))
                        } else {
                            Ok(Wgmma::Fp8Descriptor(Fp8Descriptor {
                                shape,
                                dtype,
                                atype,
                                btype,
                                destination,
                                a_descriptor: a_operand,
                                b_descriptor,
                                scale_d,
                                imm_scale_a,
                                imm_scale_b,
                            }))
                        };
                    }
                    other => {
                        return Err(unexpected_value(
                            directive_span.clone(),
                            &[".f16", ".bf16", ".tf32", ".atype"],
                            format!(".{other}"),
                        ));
                    }
                }
            }
            "s32" => {
                expect_directive_value(stream, "s32")?;

                if consume_directive_if(stream, "atype") {
                    let shape = convert_integer_shape(&shape_literal, shape_span)?;
                    let atype = parse_integer_input_type(stream)?;
                    expect_directive_value(stream, "btype")?;
                    let btype = parse_integer_input_type(stream)?;

                    let destination = RegisterOperand::parse(stream)?;
                    stream.expect(&PtxToken::Comma)?;
                    let a_operand = RegisterOperand::parse(stream)?;
                    stream.expect(&PtxToken::Comma)?;
                    let b_descriptor = RegisterOperand::parse(stream)?;
                    stream.expect(&PtxToken::Comma)?;
                    let scale_d = PredicateRegister::parse(stream)?;
                    stream.expect(&PtxToken::Semicolon)?;

                    return if is_register_vector(&a_operand) {
                        Ok(Wgmma::IntegerRegister(IntegerRegister {
                            shape,
                            satfinite,
                            atype,
                            btype,
                            destination,
                            a_register: a_operand,
                            b_descriptor,
                            scale_d,
                        }))
                    } else {
                        Ok(Wgmma::IntegerDescriptor(IntegerDescriptor {
                            shape,
                            satfinite,
                            atype,
                            btype,
                            destination,
                            a_descriptor: a_operand,
                            b_descriptor,
                            scale_d,
                        }))
                    };
                }

                expect_directive_value(stream, "b1")?;
                expect_directive_value(stream, "b1")?;
                expect_directive_value(stream, "op")?;
                let operation = parse_bit_operation(stream)?;

                let shape = convert_bit_shape(&shape_literal, shape_span)?;
                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let a_operand = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let b_descriptor = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let scale_d = PredicateRegister::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                return if is_register_vector(&a_operand) {
                    Ok(Wgmma::SingleBitRegister(SingleBitRegister {
                        shape,
                        operation,
                        destination,
                        a_register: a_operand,
                        b_descriptor,
                        scale_d,
                    }))
                } else {
                    Ok(Wgmma::SingleBitDescriptor(SingleBitDescriptor {
                        shape,
                        operation,
                        destination,
                        a_descriptor: a_operand,
                        b_descriptor,
                        scale_d,
                    }))
                };
            }
            other => Err(unexpected_value(
                directive_span,
                &[".dtype", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}
