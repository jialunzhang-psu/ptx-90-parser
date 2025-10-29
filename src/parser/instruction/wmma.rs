use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::wmma::*},
};

fn convert_layout(value: &str, span: Span) -> Result<Layout, PtxParseError> {
    match value {
        "row" => Ok(Layout::Row),
        "col" => Ok(Layout::Col),
        other => Err(unexpected_value(
            span,
            &[".row", ".col"],
            format!(".{other}"),
        )),
    }
}

fn convert_shape(value: &str, span: Span) -> Result<Shape, PtxParseError> {
    match value {
        "m16n16k16" => Ok(Shape::M16N16K16),
        "m8n32k16" => Ok(Shape::M8N32K16),
        "m32n8k16" => Ok(Shape::M32N8K16),
        "m16n16k8" => Ok(Shape::M16N16K8),
        "m8n8k4" => Ok(Shape::M8N8K4),
        "m8n8k32" => Ok(Shape::M8N8K32),
        "m8n8k128" => Ok(Shape::M8N8K128),
        other => Err(unexpected_value(
            span,
            &[
                ".m16n16k16",
                ".m8n32k16",
                ".m32n8k16",
                ".m16n16k8",
                ".m8n8k4",
                ".m8n8k32",
                ".m8n8k128",
            ],
            format!(".{other}"),
        )),
    }
}

impl PtxParser for Layout {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        convert_layout(&value, span)
    }
}

impl PtxParser for Shape {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        convert_shape(&value, span)
    }
}

impl PtxParser for StateSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        match value.as_str() {
            "global" => Ok(StateSpace::Global),
            "shared" => {
                if stream.check(|token| matches!(token, PtxToken::Colon)) {
                    stream.expect_double_colon()?;
                    let (modifier, modifier_span) = stream.expect_identifier()?;
                    if modifier == "cta" {
                        Ok(StateSpace::SharedCta)
                    } else {
                        Err(unexpected_value(modifier_span, &["cta"], modifier))
                    }
                } else {
                    Ok(StateSpace::Shared)
                }
            }
            other => Err(unexpected_value(
                span,
                &[".global", ".shared", ".shared::cta"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for AType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        match value.as_str() {
            "f16" => Ok(AType::F16),
            "s8" => Ok(AType::S8),
            "u8" => Ok(AType::U8),
            "bf16" => Ok(AType::Bf16),
            "tf32" => Ok(AType::Tf32),
            "f64" => Ok(AType::F64),
            "s4" => Ok(AType::S4),
            "u4" => Ok(AType::U4),
            "b1" => Ok(AType::B1),
            other => Err(unexpected_value(
                span,
                &[
                    ".f16", ".s8", ".u8", ".bf16", ".tf32", ".f64", ".s4", ".u4", ".b1",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for BType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        match value.as_str() {
            "f16" => Ok(BType::F16),
            "s8" => Ok(BType::S8),
            "u8" => Ok(BType::U8),
            "bf16" => Ok(BType::Bf16),
            "tf32" => Ok(BType::Tf32),
            "f64" => Ok(BType::F64),
            "s4" => Ok(BType::S4),
            "u4" => Ok(BType::U4),
            "b1" => Ok(BType::B1),
            other => Err(unexpected_value(
                span,
                &[
                    ".f16", ".s8", ".u8", ".bf16", ".tf32", ".f64", ".s4", ".u4", ".b1",
                ],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for CType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (value, span) = stream.expect_directive()?;
        match value.as_str() {
            "f16" => Ok(CType::F16),
            "f32" => Ok(CType::F32),
            "s32" => Ok(CType::S32),
            "f64" => Ok(CType::F64),
            other => Err(unexpected_value(
                span,
                &[".f16", ".f32", ".s32", ".f64"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_layout_and_shape(stream: &mut PtxTokenStream) -> Result<(Layout, Shape), PtxParseError> {
    let (first_value, first_span) = stream.expect_directive()?;
    let mut layout = match convert_layout(&first_value, first_span.clone()) {
        Ok(layout) => Some(layout),
        Err(_) => None,
    };
    let mut shape = match convert_shape(&first_value, first_span.clone()) {
        Ok(shape) => Some(shape),
        Err(_) => None,
    };

    if layout.is_none() && shape.is_none() {
        return Err(unexpected_value(
            first_span,
            &[
                ".row",
                ".col",
                ".m16n16k16",
                ".m8n32k16",
                ".m32n8k16",
                ".m16n16k8",
                ".m8n8k4",
                ".m8n8k32",
                ".m8n8k128",
            ],
            format!(".{first_value}"),
        ));
    }

    let (second_value, second_span) = stream.expect_directive()?;
    if layout.is_none() {
        layout = Some(convert_layout(&second_value, second_span.clone())?);
    } else if matches!(convert_layout(&second_value, second_span.clone()), Ok(_)) {
        return Err(unexpected_value(
            second_span,
            &[
                ".m16n16k16",
                ".m8n32k16",
                ".m32n8k16",
                ".m16n16k8",
                ".m8n8k4",
                ".m8n8k32",
                ".m8n8k128",
            ],
            format!(".{second_value}"),
        ));
    }

    if shape.is_none() {
        shape = Some(convert_shape(&second_value, second_span.clone())?);
    } else if matches!(convert_shape(&second_value, second_span.clone()), Ok(_)) {
        return Err(unexpected_value(
            second_span,
            &[".row", ".col"],
            format!(".{second_value}"),
        ));
    }

    Ok((layout.unwrap(), shape.unwrap()))
}

fn parse_optional_state_space(
    stream: &mut PtxTokenStream,
) -> Result<Option<StateSpace>, PtxParseError> {
    if let Some((modifier, _)) = peek_directive(stream)? {
        if modifier == "global" || modifier == "shared" {
            StateSpace::parse(stream).map(Some)
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn parse_optional_stride(stream: &mut PtxTokenStream) -> Result<Option<Operand>, PtxParseError> {
    if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Ok(Some(Operand::parse(stream)?))
    } else {
        Ok(None)
    }
}

fn parse_load_a(stream: &mut PtxTokenStream) -> Result<LoadA, PtxParseError> {
    expect_directive_value(stream, "sync")?;
    expect_directive_value(stream, "aligned")?;

    let (layout, shape) = parse_layout_and_shape(stream)?;
    let state_space = parse_optional_state_space(stream)?;
    let data_type = AType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    let stride = parse_optional_stride(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(LoadA {
        layout,
        shape,
        state_space,
        data_type,
        destination,
        address,
        stride,
    })
}

fn parse_load_b(stream: &mut PtxTokenStream) -> Result<LoadB, PtxParseError> {
    expect_directive_value(stream, "sync")?;
    expect_directive_value(stream, "aligned")?;

    let (layout, shape) = parse_layout_and_shape(stream)?;
    let state_space = parse_optional_state_space(stream)?;
    let data_type = BType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    let stride = parse_optional_stride(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(LoadB {
        layout,
        shape,
        state_space,
        data_type,
        destination,
        address,
        stride,
    })
}

fn parse_load_c(stream: &mut PtxTokenStream) -> Result<LoadC, PtxParseError> {
    expect_directive_value(stream, "sync")?;
    expect_directive_value(stream, "aligned")?;

    let (layout, shape) = parse_layout_and_shape(stream)?;
    let state_space = parse_optional_state_space(stream)?;
    let data_type = CType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    let stride = parse_optional_stride(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(LoadC {
        layout,
        shape,
        state_space,
        data_type,
        destination,
        address,
        stride,
    })
}

impl PtxParser for Instruction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "wmma")?;
        expect_directive_value(stream, "load")?;

        let (variant, span) = stream.expect_directive()?;
        match variant.as_str() {
            "a" => Ok(Instruction::LoadA(parse_load_a(stream)?)),
            "b" => Ok(Instruction::LoadB(parse_load_b(stream)?)),
            "c" => Ok(Instruction::LoadC(parse_load_c(stream)?)),
            other => Err(unexpected_value(
                span,
                &[".a", ".b", ".c"],
                format!(".{other}"),
            )),
        }
    }
}
