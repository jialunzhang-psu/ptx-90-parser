use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, Span, consume_directive_if,
        expect_directive_value, expect_identifier_value, invalid_literal, parse_u16_literal,
        peek_directive, unexpected_value,
    },
    r#type::{common::RegisterOperand, instruction::mma::*},
};

fn parse_layout_with_span(stream: &mut PtxTokenStream) -> Result<(Layout, Span), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "row" => Ok((Layout::Row, span)),
        "col" => Ok((Layout::Col, span)),
        other => Err(unexpected_value(
            span,
            &[".row", ".col"],
            format!(".{other}"),
        )),
    }
}

fn parse_layout(stream: &mut PtxTokenStream) -> Result<Layout, PtxParseError> {
    parse_layout_with_span(stream).map(|(value, _)| value)
}

fn parse_data_type_with_span(
    stream: &mut PtxTokenStream,
) -> Result<(DataType, Span), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "f16" => Ok((DataType::F16, span)),
        "f32" => Ok((DataType::F32, span)),
        other => Err(unexpected_value(
            span,
            &[".f16", ".f32"],
            format!(".{other}"),
        )),
    }
}

#[allow(dead_code)]
fn parse_data_type(stream: &mut PtxTokenStream) -> Result<DataType, PtxParseError> {
    parse_data_type_with_span(stream).map(|(value, _)| value)
}

fn parse_alternate_type_with_span(
    stream: &mut PtxTokenStream,
) -> Result<(AlternateMatrixType, Span), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "bf16" => Ok((AlternateMatrixType::Bf16, span)),
        "tf32" => Ok((AlternateMatrixType::Tf32, span)),
        other => Err(unexpected_value(
            span,
            &[".bf16", ".tf32"],
            format!(".{other}"),
        )),
    }
}

fn parse_alternate_type(stream: &mut PtxTokenStream) -> Result<AlternateMatrixType, PtxParseError> {
    parse_alternate_type_with_span(stream).map(|(value, _)| value)
}

fn parse_f8_type_with_span(stream: &mut PtxTokenStream) -> Result<(F8Type, Span), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "e4m3" => Ok((F8Type::E4M3, span)),
        "e5m2" => Ok((F8Type::E5M2, span)),
        other => Err(unexpected_value(
            span,
            &[".e4m3", ".e5m2"],
            format!(".{other}"),
        )),
    }
}

#[allow(dead_code)]
fn parse_f8_type(stream: &mut PtxTokenStream) -> Result<F8Type, PtxParseError> {
    parse_f8_type_with_span(stream).map(|(value, _)| value)
}

fn parse_f8f6f4_type_with_span(
    stream: &mut PtxTokenStream,
) -> Result<(F8F6F4Type, Span), PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "e4m3" => Ok((F8F6F4Type::E4M3, span)),
        "e5m2" => Ok((F8F6F4Type::E5M2, span)),
        "e3m2" => Ok((F8F6F4Type::E3M2, span)),
        "e2m3" => Ok((F8F6F4Type::E2M3, span)),
        "e2m1" => Ok((F8F6F4Type::E2M1, span)),
        other => Err(unexpected_value(
            span,
            &[".e4m3", ".e5m2", ".e3m2", ".e2m3", ".e2m1"],
            format!(".{other}"),
        )),
    }
}

fn parse_f8f6f4_type(stream: &mut PtxTokenStream) -> Result<F8F6F4Type, PtxParseError> {
    parse_f8f6f4_type_with_span(stream).map(|(value, _)| value)
}

#[allow(dead_code)]
fn parse_f8_shape(stream: &mut PtxTokenStream) -> Result<F8Shape, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    convert_f8_shape(&value, span)
}

fn convert_f8_shape(value: &str, span: Span) -> Result<F8Shape, PtxParseError> {
    match value {
        "m16n8k16" => Ok(F8Shape::M16N8K16),
        "m16n8k32" => Ok(F8Shape::M16N8K32),
        other => Err(unexpected_value(
            span,
            &[".m16n8k16", ".m16n8k32"],
            format!(".{other}"),
        )),
    }
}

#[allow(dead_code)]
fn parse_f64_shape(stream: &mut PtxTokenStream) -> Result<F64Shape, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    convert_f64_shape(&value, span)
}

fn convert_f64_shape(value: &str, span: Span) -> Result<F64Shape, PtxParseError> {
    match value {
        "m8n8k4" => Ok(F64Shape::M8N8K4),
        "m16n8k4" => Ok(F64Shape::M16N8K4),
        "m16n8k8" => Ok(F64Shape::M16N8K8),
        "m16n8k16" => Ok(F64Shape::M16N8K16),
        other => Err(unexpected_value(
            span,
            &[".m8n8k4", ".m16n8k4", ".m16n8k8", ".m16n8k16"],
            format!(".{other}"),
        )),
    }
}

#[allow(dead_code)]
fn parse_integer8_shape(stream: &mut PtxTokenStream) -> Result<Integer8Shape, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    convert_integer8_shape(&value, span)
}

fn convert_integer8_shape(value: &str, span: Span) -> Result<Integer8Shape, PtxParseError> {
    match value {
        "m8n8k16" => Ok(Integer8Shape::M8N8K16),
        "m16n8k16" => Ok(Integer8Shape::M16N8K16),
        "m16n8k32" => Ok(Integer8Shape::M16N8K32),
        other => Err(unexpected_value(
            span,
            &[".m8n8k16", ".m16n8k16", ".m16n8k32"],
            format!(".{other}"),
        )),
    }
}

#[allow(dead_code)]
fn parse_integer4_shape(stream: &mut PtxTokenStream) -> Result<Integer4Shape, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    convert_integer4_shape(&value, span)
}

fn convert_integer4_shape(value: &str, span: Span) -> Result<Integer4Shape, PtxParseError> {
    match value {
        "m8n8k32" => Ok(Integer4Shape::M8N8K32),
        "m16n8k32" => Ok(Integer4Shape::M16N8K32),
        "m16n8k64" => Ok(Integer4Shape::M16N8K64),
        other => Err(unexpected_value(
            span,
            &[".m8n8k32", ".m16n8k32", ".m16n8k64"],
            format!(".{other}"),
        )),
    }
}

fn parse_integer8_type(stream: &mut PtxTokenStream) -> Result<Integer8Type, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "u8" => Ok(Integer8Type::U8),
        "s8" => Ok(Integer8Type::S8),
        other => Err(unexpected_value(span, &[".u8", ".s8"], format!(".{other}"))),
    }
}

fn parse_integer4_type(stream: &mut PtxTokenStream) -> Result<Integer4Type, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "u4" => Ok(Integer4Type::U4),
        "s4" => Ok(Integer4Type::S4),
        other => Err(unexpected_value(span, &[".u4", ".s4"], format!(".{other}"))),
    }
}
#[allow(dead_code)]
fn parse_single_bit_shape(stream: &mut PtxTokenStream) -> Result<SingleBitShape, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    convert_single_bit_shape(&value, span)
}

fn convert_single_bit_shape(value: &str, span: Span) -> Result<SingleBitShape, PtxParseError> {
    match value {
        "m8n8k128" => Ok(SingleBitShape::M8N8K128),
        "m16n8k128" => Ok(SingleBitShape::M16N8K128),
        "m16n8k256" => Ok(SingleBitShape::M16N8K256),
        other => Err(unexpected_value(
            span,
            &[".m8n8k128", ".m16n8k128", ".m16n8k256"],
            format!(".{other}"),
        )),
    }
}

fn parse_bit_op(stream: &mut PtxTokenStream) -> Result<BitOp, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "xor" => Ok(BitOp::Xor),
        "and" => Ok(BitOp::And),
        other => Err(unexpected_value(
            span,
            &[".xor", ".and"],
            format!(".{other}"),
        )),
    }
}

fn parse_scale_vec_value(stream: &mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    let (token, span) = stream.peek()?;
    match token {
        PtxToken::Directive(value) => {
            let span = span.clone();
            let value = value.clone();
            stream.consume()?;
            Ok((value, span))
        }
        PtxToken::Identifier(value) => {
            let span = span.clone();
            let value = value.clone();
            stream.consume()?;
            Ok((value, span))
        }
        PtxToken::DecimalInteger(number) => {
            let mut span = span.clone();
            let mut value = number.clone();
            stream.consume()?;
            if let Some((token, suffix_span)) =
                stream.consume_if(|token| matches!(token, PtxToken::Identifier(_)))
            {
                if let PtxToken::Identifier(suffix) = token {
                    value.push_str(suffix);
                    span.end = suffix_span.end;
                }
            }
            Ok((value, span))
        }
        _ => Err(unexpected_value(
            span.clone(),
            &["scale vector literal"],
            format!("{token:?}"),
        )),
    }
}

fn parse_mxf4_scale_vec_size(
    stream: &mut PtxTokenStream,
) -> Result<MxF4ScaleVecSize, PtxParseError> {
    stream.expect_double_colon()?;
    let (value, span) = parse_scale_vec_value(stream)?;
    match value.as_str() {
        "2X" | "2x" => Ok(MxF4ScaleVecSize::TwoX),
        other => Err(unexpected_value(span, &["::2X"], format!("::{other}"))),
    }
}

fn parse_mxf4nvf4_scale_vec_size(
    stream: &mut PtxTokenStream,
) -> Result<MxF4NvF4ScaleVecSize, PtxParseError> {
    stream.expect_double_colon()?;
    let (value, span) = parse_scale_vec_value(stream)?;
    match value.as_str() {
        "2X" | "2x" => Ok(MxF4NvF4ScaleVecSize::TwoX),
        "4X" | "4x" => Ok(MxF4NvF4ScaleVecSize::FourX),
        other => Err(unexpected_value(
            span,
            &["::2X", "::4X"],
            format!("::{other}"),
        )),
    }
}

fn parse_mxf8f6f4_scale_vec_size(
    stream: &mut PtxTokenStream,
) -> Result<MxF8F6F4ScaleVecSize, PtxParseError> {
    stream.expect_double_colon()?;
    let (value, span) = parse_scale_vec_value(stream)?;
    match value.as_str() {
        "1X" | "1x" => Ok(MxF8F6F4ScaleVecSize::OneX),
        other => Err(unexpected_value(span, &["::1X"], format!("::{other}"))),
    }
}

fn parse_mxf4_scale_dtype(stream: &mut PtxTokenStream) -> Result<MxF4ScaleDataType, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "ue8m0" => Ok(MxF4ScaleDataType::UE8M0),
        other => Err(unexpected_value(span, &[".ue8m0"], format!(".{other}"))),
    }
}

fn parse_mxf4nvf4_scale_dtype(
    stream: &mut PtxTokenStream,
) -> Result<MxF4NvF4ScaleDataType, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "ue8m0" => Ok(MxF4NvF4ScaleDataType::UE8M0),
        "ue4m3" => Ok(MxF4NvF4ScaleDataType::UE4M3),
        other => Err(unexpected_value(
            span,
            &[".ue8m0", ".ue4m3"],
            format!(".{other}"),
        )),
    }
}

fn parse_mxf8f6f4_scale_dtype(
    stream: &mut PtxTokenStream,
) -> Result<MxF8F6F4ScaleDataType, PtxParseError> {
    let (value, span) = stream.expect_directive()?;
    match value.as_str() {
        "ue8m0" => Ok(MxF8F6F4ScaleDataType::UE8M0),
        other => Err(unexpected_value(span, &[".ue8m0"], format!(".{other}"))),
    }
}

fn parse_kind_value(stream: &mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    let mut span = None;
    if stream
        .consume_if(|token| matches!(token, PtxToken::Colon))
        .is_some()
    {
        let (_, second) = stream.expect(&PtxToken::Colon)?;
        span = Some(second.clone());
    }

    if let Some((token, token_span)) =
        stream.consume_if(|token| matches!(token, PtxToken::Directive(_)))
    {
        if span.is_none() {
            span = Some(token_span.clone());
        }
        if let PtxToken::Directive(value) = token {
            return Ok((value.clone(), token_span.clone()));
        }
    }

    if let Some((token, token_span)) =
        stream.consume_if(|token| matches!(token, PtxToken::Identifier(_)))
    {
        if let PtxToken::Identifier(value) = token {
            return Ok((value.clone(), token_span.clone()));
        }
    }

    Err(match span {
        Some(span) => unexpected_value(span, &["namespaced identifier"], "invalid namespace value"),
        None => crate::parser::PtxParseError {
            kind: crate::parser::ParseErrorKind::UnexpectedEof,
            span: 0..0,
        },
    })
}

fn parse_block_scale_operands(
    stream: &mut PtxTokenStream,
) -> Result<BlockScaleOperands, PtxParseError> {
    let scale_a_data = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    stream.expect(&PtxToken::LBrace)?;
    let (scale_a_byte_id, _) = parse_u16_literal(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (scale_a_thread_id, _) = parse_u16_literal(stream)?;
    stream.expect(&PtxToken::RBrace)?;

    stream.expect(&PtxToken::Comma)?;
    let scale_b_data = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;

    stream.expect(&PtxToken::LBrace)?;
    let (scale_b_byte_id, _) = parse_u16_literal(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (scale_b_thread_id, _) = parse_u16_literal(stream)?;
    stream.expect(&PtxToken::RBrace)?;

    Ok(BlockScaleOperands {
        scale_a_data,
        scale_a_byte_id,
        scale_a_thread_id,
        scale_b_data,
        scale_b_byte_id,
        scale_b_thread_id,
    })
}

fn parse_register_operands(
    stream: &mut PtxTokenStream,
) -> Result<
    (
        RegisterOperand,
        RegisterOperand,
        RegisterOperand,
        RegisterOperand,
    ),
    PtxParseError,
> {
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let operand_a = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let operand_b = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let operand_c = RegisterOperand::parse(stream)?;
    Ok((destination, operand_a, operand_b, operand_c))
}

fn ensure_constant_data_type(
    span: Span,
    value: DataType,
    expected: DataType,
    context: &str,
) -> Result<(), PtxParseError> {
    if value == expected {
        Ok(())
    } else {
        Err(invalid_literal(
            span,
            format!(
                "expected {context} to use {:?}, found {:?}",
                expected, value
            ),
        ))
    }
}

fn ensure_alternate_type(
    span: Span,
    value: AlternateMatrixType,
    expected: AlternateMatrixType,
    context: &str,
) -> Result<(), PtxParseError> {
    if value == expected {
        Ok(())
    } else {
        Err(invalid_literal(
            span,
            format!(
                "expected {context} to use {:?}, found {:?}",
                expected, value
            ),
        ))
    }
}

fn ensure_f8f6f4_type(
    span: Span,
    value: F8F6F4Type,
    expected: F8F6F4Type,
    context: &str,
) -> Result<(), PtxParseError> {
    if value == expected {
        Ok(())
    } else {
        Err(invalid_literal(
            span,
            format!(
                "expected {context} to use {:?}, found {:?}",
                expected, value
            ),
        ))
    }
}

fn parse_sync_aligned_m8n8k4(stream: &mut PtxTokenStream) -> Result<MmaInstruction, PtxParseError> {
    consume_directive_if(stream, "alayout");
    let a_layout = parse_layout(stream)?;
    consume_directive_if(stream, "blayout");
    let b_layout = parse_layout(stream)?;

    consume_directive_if(stream, "dtype");
    let (d_type, _) = parse_data_type_with_span(stream)?;

    for _ in 0..2 {
        if let Some((next, _span)) = peek_directive(stream)? {
            if next == "ctype" {
                break;
            }

            if next == "f16" || next == "f32" {
                let (value, value_span) = parse_data_type_with_span(stream)?;
                ensure_constant_data_type(value_span, value, DataType::F16, "multiplicand data")?;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    consume_directive_if(stream, "ctype");
    let (c_type, _) = parse_data_type_with_span(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedM8N8K4(SyncAlignedM8N8K4 {
        a_layout,
        b_layout,
        d_type,
        c_type,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn expect_row_col_layouts(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    let (first, span_first) = parse_layout_with_span(stream)?;
    if first != Layout::Row {
        return Err(invalid_literal(
            span_first,
            "expected matrix A layout to be .row",
        ));
    }
    let (second, span_second) = parse_layout_with_span(stream)?;
    if second != Layout::Col {
        return Err(invalid_literal(
            span_second,
            "expected matrix B layout to be .col",
        ));
    }
    Ok(())
}

fn parse_sync_aligned_m16n8k8(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;

    if consume_directive_if(stream, "dtype") {
        // label consumed, nothing else required
    }
    let (d_type, _) = parse_data_type_with_span(stream)?;

    let next_directive = peek_directive(stream)?;
    let is_mixed = next_directive
        .as_ref()
        .map(|(value, _)| matches!(value.as_str(), "atype" | "bf16" | "tf32"))
        .unwrap_or(false);

    if is_mixed
        || matches!(d_type, DataType::F32)
            && next_directive
                .as_ref()
                .map(|(value, _)| matches!(value.as_str(), "bf16" | "tf32" | "atype"))
                .unwrap_or(false)
    {
        consume_directive_if(stream, "atype");
        let a_type = parse_alternate_type(stream)?;
        consume_directive_if(stream, "btype");
        let b_type = parse_alternate_type(stream)?;
        consume_directive_if(stream, "ctype");
        let (_c_type, _) = parse_data_type_with_span(stream)?;

        let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;

        stream.expect(&PtxToken::Semicolon)?;
        return Ok(MmaInstruction::SyncAlignedM16N8K8Mixed(
            SyncAlignedM16N8K8Mixed {
                a_type,
                b_type,
                destination,
                operand_a,
                operand_b,
                operand_c,
            },
        ));
    }

    for _ in 0..2 {
        if let Some((next, _)) = peek_directive(stream)? {
            if next == "ctype" {
                break;
            }
            if next == "f16" || next == "f32" {
                let (value, span) = parse_data_type_with_span(stream)?;
                ensure_constant_data_type(span, value, DataType::F16, "multiplicand data")?;
            } else {
                break;
            }
        }
    }

    consume_directive_if(stream, "ctype");
    let (c_type, _) = parse_data_type_with_span(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedM16N8K8(SyncAlignedM16N8K8 {
        d_type,
        c_type,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn parse_sync_aligned_m16n8k16(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;

    if consume_directive_if(stream, "dtype") {
        // label consumed
    }
    let (d_type, _) = parse_data_type_with_span(stream)?;

    let next_directive = peek_directive(stream)?;
    let mut is_bf16_variant = false;
    if matches!(d_type, DataType::F32) {
        if let Some((value, _)) = next_directive.as_ref() {
            if value == "bf16" {
                is_bf16_variant = true;
            }
        }
    }

    if is_bf16_variant {
        let (a_type, a_span) = parse_alternate_type_with_span(stream)?;
        let (b_type, b_span) = parse_alternate_type_with_span(stream)?;
        if a_type != AlternateMatrixType::Bf16 {
            return Err(invalid_literal(
                a_span,
                "expected multiplicand A type to be .bf16",
            ));
        }
        if b_type != AlternateMatrixType::Bf16 {
            return Err(invalid_literal(
                b_span,
                "expected multiplicand B type to be .bf16",
            ));
        }
        consume_directive_if(stream, "ctype");
        let (c_type, span) = parse_data_type_with_span(stream)?;
        ensure_constant_data_type(span, c_type, DataType::F32, "accumulator type")?;

        let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        return Ok(MmaInstruction::SyncAlignedM16N8K16Bf16(
            SyncAlignedM16N8K16Bf16 {
                destination,
                operand_a,
                operand_b,
                operand_c,
            },
        ));
    }

    for _ in 0..2 {
        if let Some((next, _)) = peek_directive(stream)? {
            if next == "ctype" {
                break;
            }
            if next == "f16" || next == "f32" {
                let (value, span) = parse_data_type_with_span(stream)?;
                ensure_constant_data_type(span, value, DataType::F16, "multiplicand data")?;
            } else {
                break;
            }
        }
    }

    consume_directive_if(stream, "ctype");
    let (c_type, _) = parse_data_type_with_span(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedM16N8K16(SyncAlignedM16N8K16 {
        d_type,
        c_type,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn parse_sync_aligned_m16n8k4_tf32(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;

    let (d_type, d_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(d_span, d_type, DataType::F32, "destination type")?;
    let (a_type, a_span) = parse_alternate_type_with_span(stream)?;
    ensure_alternate_type(
        a_span,
        a_type,
        AlternateMatrixType::Tf32,
        "multiplicand A type",
    )?;
    let (b_type, b_span) = parse_alternate_type_with_span(stream)?;
    ensure_alternate_type(
        b_span,
        b_type,
        AlternateMatrixType::Tf32,
        "multiplicand B type",
    )?;
    let (c_type, c_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(c_span, c_type, DataType::F32, "accumulator type")?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedM16N8K4Tf32(
        SyncAlignedM16N8K4Tf32 {
            destination,
            operand_a,
            operand_b,
            operand_c,
        },
    ))
}

fn parse_sync_aligned_m16n8k32_f8f6f4(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;
    expect_directive_value(stream, "kind")?;
    let (kind_value, kind_span) = parse_kind_value(stream)?;
    if kind_value != "f8f6f4" {
        return Err(invalid_literal(
            kind_span,
            format!("expected kind::f8f6f4, found {kind_value}"),
        ));
    }

    consume_directive_if(stream, "dtype");
    let (d_type, _) = parse_data_type_with_span(stream)?;
    consume_directive_if(stream, "f8f6f4type");
    let a_type = parse_f8f6f4_type(stream)?;
    consume_directive_if(stream, "f8f6f4type");
    let b_type = parse_f8f6f4_type(stream)?;
    consume_directive_if(stream, "ctype");
    let (c_type, _) = parse_data_type_with_span(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedM16N8K32F8F6F4(
        SyncAlignedM16N8K32F8F6F4 {
            d_type,
            a_type,
            b_type,
            c_type,
            destination,
            operand_a,
            operand_b,
            operand_c,
        },
    ))
}

fn parse_block_scale_suffix(
    stream: &mut PtxTokenStream,
) -> Result<BlockScaleOperands, PtxParseError> {
    stream.expect(&PtxToken::Comma)?;
    let block_scale = parse_block_scale_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(block_scale)
}

fn parse_sync_aligned_block_scale_m16n8k64_mxf4(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;
    expect_directive_value(stream, "kind")?;
    let (kind, span) = parse_kind_value(stream)?;
    if kind != "mxf4" {
        return Err(invalid_literal(
            span,
            format!("expected kind::mxf4, found {kind}"),
        ));
    }

    expect_directive_value(stream, "block_scale")?;
    let scale_vec_size = if consume_directive_if(stream, "scale_vec")
        || consume_directive_if(stream, "scale_vec_size")
    {
        Some(parse_mxf4_scale_vec_size(stream)?)
    } else {
        None
    };

    let (d_type, d_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(d_span, d_type, DataType::F32, "destination type")?;

    let (a_type, a_span) = parse_f8f6f4_type_with_span(stream)?;
    ensure_f8f6f4_type(a_span, a_type, F8F6F4Type::E2M1, "multiplicand A type")?;
    let (b_type, b_span) = parse_f8f6f4_type_with_span(stream)?;
    ensure_f8f6f4_type(b_span, b_type, F8F6F4Type::E2M1, "multiplicand B type")?;
    let (c_type, c_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(c_span, c_type, DataType::F32, "accumulator type")?;
    consume_directive_if(stream, "stype");
    let stype = parse_mxf4_scale_dtype(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    let block_scale = parse_block_scale_suffix(stream)?;

    Ok(MmaInstruction::SyncAlignedBlockScaleM16N8K64MxF4(
        SyncAlignedBlockScaleM16N8K64MxF4 {
            scale_vec_size,
            stype,
            destination,
            operand_a,
            operand_b,
            operand_c,
            block_scale,
        },
    ))
}

fn parse_sync_aligned_block_scale_m16n8k64_mxf4nvf4(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;
    expect_directive_value(stream, "kind")?;
    let (kind, span) = parse_kind_value(stream)?;
    if kind != "mxf4nvf4" {
        return Err(invalid_literal(
            span,
            format!("expected kind::mxf4nvf4, found {kind}"),
        ));
    }

    expect_directive_value(stream, "block_scale")?;
    if !(consume_directive_if(stream, "scale_vec")
        || consume_directive_if(stream, "scale_vec_size"))
    {
        return Err(unexpected_value(
            span,
            &[".scale_vec::2X", ".scale_vec::4X"],
            "missing .scale_vec_size modifier",
        ));
    }
    let scale_vec_size = parse_mxf4nvf4_scale_vec_size(stream)?;

    let (d_type, d_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(d_span, d_type, DataType::F32, "destination type")?;
    let (a_type, a_span) = parse_f8f6f4_type_with_span(stream)?;
    ensure_f8f6f4_type(a_span, a_type, F8F6F4Type::E2M1, "multiplicand A type")?;
    let (b_type, b_span) = parse_f8f6f4_type_with_span(stream)?;
    ensure_f8f6f4_type(b_span, b_type, F8F6F4Type::E2M1, "multiplicand B type")?;
    let (c_type, c_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(c_span, c_type, DataType::F32, "accumulator type")?;
    consume_directive_if(stream, "stype");
    let stype = parse_mxf4nvf4_scale_dtype(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    let block_scale = parse_block_scale_suffix(stream)?;

    Ok(MmaInstruction::SyncAlignedBlockScaleM16N8K64MxF4NvF4(
        SyncAlignedBlockScaleM16N8K64MxF4NvF4 {
            scale_vec_size,
            stype,
            destination,
            operand_a,
            operand_b,
            operand_c,
            block_scale,
        },
    ))
}

fn parse_shape_f8(
    stream: &mut PtxTokenStream,
    shape_value: &str,
    shape_span: Span,
) -> Result<MmaInstruction, PtxParseError> {
    let shape = convert_f8_shape(shape_value, shape_span)?;
    consume_directive_if(stream, "dtype");
    let (d_type, _) = parse_data_type_with_span(stream)?;
    consume_directive_if(stream, "f8type");
    let (a_type, _) = parse_f8_type_with_span(stream)?;
    consume_directive_if(stream, "f8type");
    let (b_type, _) = parse_f8_type_with_span(stream)?;
    consume_directive_if(stream, "ctype");
    let (c_type, _) = parse_data_type_with_span(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedF8(SyncAlignedF8 {
        shape,
        d_type,
        a_type,
        b_type,
        c_type,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn parse_shape_f64(
    stream: &mut PtxTokenStream,
    shape_value: &str,
    shape_span: Span,
) -> Result<MmaInstruction, PtxParseError> {
    let shape = convert_f64_shape(shape_value, shape_span)?;

    expect_directive_value(stream, "f64")?;
    expect_directive_value(stream, "f64")?;
    expect_directive_value(stream, "f64")?;
    expect_directive_value(stream, "f64")?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedF64(SyncAlignedF64 {
        shape,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn parse_shape_integer8(
    stream: &mut PtxTokenStream,
    shape_value: &str,
    shape_span: Span,
) -> Result<MmaInstruction, PtxParseError> {
    let shape = convert_integer8_shape(shape_value, shape_span)?;
    let satfinite = consume_directive_if(stream, "satfinite");

    consume_directive_if(stream, "dtype");
    expect_directive_value(stream, "s32")?;
    consume_directive_if(stream, "atype");
    let a_type = parse_integer8_type(stream)?;
    consume_directive_if(stream, "btype");
    let b_type = parse_integer8_type(stream)?;
    consume_directive_if(stream, "ctype");
    expect_directive_value(stream, "s32")?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedInteger8Bit(
        SyncAlignedInteger8Bit {
            shape,
            satfinite,
            a_type,
            b_type,
            destination,
            operand_a,
            operand_b,
            operand_c,
        },
    ))
}

fn parse_shape_integer4(
    stream: &mut PtxTokenStream,
    shape_value: &str,
    shape_span: Span,
) -> Result<MmaInstruction, PtxParseError> {
    let shape = convert_integer4_shape(shape_value, shape_span)?;
    let satfinite = consume_directive_if(stream, "satfinite");

    consume_directive_if(stream, "dtype");
    expect_directive_value(stream, "s32")?;
    consume_directive_if(stream, "atype");
    let a_type = parse_integer4_type(stream)?;
    consume_directive_if(stream, "btype");
    let b_type = parse_integer4_type(stream)?;
    consume_directive_if(stream, "ctype");
    expect_directive_value(stream, "s32")?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedInteger4Bit(
        SyncAlignedInteger4Bit {
            shape,
            satfinite,
            a_type,
            b_type,
            destination,
            operand_a,
            operand_b,
            operand_c,
        },
    ))
}

fn parse_shape_single_bit(
    stream: &mut PtxTokenStream,
    shape_value: &str,
    shape_span: Span,
) -> Result<MmaInstruction, PtxParseError> {
    let shape = convert_single_bit_shape(shape_value, shape_span)?;

    expect_directive_value(stream, "s32")?;
    expect_directive_value(stream, "b1")?;
    expect_directive_value(stream, "b1")?;
    expect_directive_value(stream, "s32")?;
    consume_directive_if(stream, "bitOp");
    let bit_op = parse_bit_op(stream)?;
    expect_directive_value(stream, "popc")?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(MmaInstruction::SyncAlignedSingleBit(SyncAlignedSingleBit {
        shape,
        bit_op,
        destination,
        operand_a,
        operand_b,
        operand_c,
    }))
}

fn parse_shape_variant(stream: &mut PtxTokenStream) -> Result<MmaInstruction, PtxParseError> {
    let (shape_value, shape_span) = stream.expect_directive()?;
    expect_row_col_layouts(stream)?;

    let next = peek_directive(stream)?;
    if let Some((next_value, _)) = next {
        match next_value.as_str() {
            "dtype" | "f16" | "f32" => {
                return parse_shape_f8(stream, &shape_value, shape_span);
            }
            "f64" => {
                return parse_shape_f64(stream, &shape_value, shape_span);
            }
            "satfinite" | "s32" => {
                // fall through to integer or single bit detection
            }
            _ => {}
        }
    }

    if matches!(shape_value.as_str(), "m8n8k128" | "m16n8k128" | "m16n8k256") {
        return parse_shape_single_bit(stream, &shape_value, shape_span);
    }

    if matches!(shape_value.as_str(), "m8n8k16" | "m16n8k16" | "m16n8k32") {
        return parse_shape_integer8(stream, &shape_value, shape_span);
    }

    if matches!(shape_value.as_str(), "m8n8k32" | "m16n8k32" | "m16n8k64") {
        return parse_shape_integer4(stream, &shape_value, shape_span);
    }

    Err(invalid_literal(
        shape_span,
        format!("unsupported mma shape variant: {shape_value}"),
    ))
}

fn parse_m16n8k32_variant(stream: &mut PtxTokenStream) -> Result<MmaInstruction, PtxParseError> {
    let position = stream.position();
    if let Err(err) = expect_row_col_layouts(stream) {
        return Err(err);
    }
    expect_directive_value(stream, "kind")?;
    let (kind_value, kind_span) = parse_kind_value(stream)?;
    stream.set_position(position);

    match kind_value.as_str() {
        "f8f6f4" => parse_sync_aligned_m16n8k32_f8f6f4(stream),
        "mxf8f6f4" => parse_sync_aligned_block_scale_m16n8k32_mxf8f6f4(stream),
        other => Err(invalid_literal(
            kind_span,
            format!("unsupported m16n8k32 kind {other}"),
        )),
    }
}

fn parse_m16n8k64_variant(stream: &mut PtxTokenStream) -> Result<MmaInstruction, PtxParseError> {
    let position = stream.position();
    if let Err(err) = expect_row_col_layouts(stream) {
        return Err(err);
    }
    expect_directive_value(stream, "kind")?;
    let (kind_value, kind_span) = parse_kind_value(stream)?;
    stream.set_position(position);

    match kind_value.as_str() {
        "mxf4" => parse_sync_aligned_block_scale_m16n8k64_mxf4(stream),
        "mxf4nvf4" => parse_sync_aligned_block_scale_m16n8k64_mxf4nvf4(stream),
        other => Err(invalid_literal(
            kind_span,
            format!("unsupported m16n8k64 kind {other}"),
        )),
    }
}

fn parse_sync_aligned_block_scale_m16n8k32_mxf8f6f4(
    stream: &mut PtxTokenStream,
) -> Result<MmaInstruction, PtxParseError> {
    expect_row_col_layouts(stream)?;
    expect_directive_value(stream, "kind")?;
    let (kind, span) = parse_kind_value(stream)?;
    if kind != "mxf8f6f4" {
        return Err(invalid_literal(
            span,
            format!("expected kind::mxf8f6f4, found {kind}"),
        ));
    }

    expect_directive_value(stream, "block_scale")?;
    let scale_vec_size = if consume_directive_if(stream, "scale_vec")
        || consume_directive_if(stream, "scale_vec_size")
    {
        Some(parse_mxf8f6f4_scale_vec_size(stream)?)
    } else {
        None
    };

    let (d_type, d_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(d_span, d_type, DataType::F32, "destination type")?;
    consume_directive_if(stream, "f8f6f4type");
    let a_type = parse_f8f6f4_type(stream)?;
    consume_directive_if(stream, "f8f6f4type");
    let b_type = parse_f8f6f4_type(stream)?;
    let (c_type, c_span) = parse_data_type_with_span(stream)?;
    ensure_constant_data_type(c_span, c_type, DataType::F32, "accumulator type")?;
    consume_directive_if(stream, "stype");
    let stype = parse_mxf8f6f4_scale_dtype(stream)?;

    let (destination, operand_a, operand_b, operand_c) = parse_register_operands(stream)?;
    let block_scale = parse_block_scale_suffix(stream)?;

    Ok(MmaInstruction::SyncAlignedBlockScaleM16N8K32MxF8F6F4(
        SyncAlignedBlockScaleM16N8K32MxF8F6F4 {
            scale_vec_size,
            a_type,
            b_type,
            stype,
            destination,
            operand_a,
            operand_b,
            operand_c,
            block_scale,
        },
    ))
}

// Placeholder for main parser implementation that will be filled in below.
impl PtxParser for MmaInstruction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "mma")?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;
        let (modifier, span) = stream.expect_directive()?;

        match modifier.as_str() {
            "m8n8k4" => parse_sync_aligned_m8n8k4(stream),
            "m16n8k8" => parse_sync_aligned_m16n8k8(stream),
            "m16n8k16" => parse_sync_aligned_m16n8k16(stream),
            "m16n8k4" => parse_sync_aligned_m16n8k4_tf32(stream),
            "m16n8k32" => parse_m16n8k32_variant(stream),
            "m16n8k64" => parse_m16n8k64_variant(stream),
            "shape" => parse_shape_variant(stream),
            other => Err(unexpected_value(
                span,
                &[
                    ".m8n8k4",
                    ".m16n8k8",
                    ".m16n8k16",
                    ".m16n8k4",
                    ".m16n8k32",
                    ".m16n8k64",
                    ".shape",
                ],
                format!(".{other}"),
            )),
        }
    }
}
