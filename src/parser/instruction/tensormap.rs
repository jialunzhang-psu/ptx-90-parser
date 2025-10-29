use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::{AddressOperand, RegisterOperand},
        instruction::tensormap::*,
    },
};

use crate::parser::common::parse_u64_literal;

fn parse_optional_state_space(
    stream: &mut PtxTokenStream,
) -> Result<Option<StateSpace>, PtxParseError> {
    if let Some((directive, _)) = peek_directive(stream)? {
        if directive == "global" || directive == "shared" {
            return Ok(Some(StateSpace::parse(stream)?));
        }
    }
    Ok(None)
}

fn parse_u32_literal(stream: &mut PtxTokenStream) -> Result<(u32, Span), PtxParseError> {
    let (value, span) = parse_u64_literal(stream)?;
    if value > u32::MAX as u64 {
        return Err(invalid_literal(
            span.clone(),
            format!("value {value} exceeds u32 range"),
        ));
    }
    Ok((value as u32, span))
}

fn parse_data_type_with_span(
    stream: &mut PtxTokenStream,
) -> Result<(DataType, Span), PtxParseError> {
    let (directive, span) = stream.expect_directive()?;
    let data_type = match directive.as_str() {
        "b32" => DataType::B32,
        "b64" => DataType::B64,
        other => {
            return Err(unexpected_value(
                span,
                &[".b32", ".b64"],
                format!(".{other}"),
            ));
        }
    };
    Ok((data_type, span))
}

fn expect_data_type(
    data_type: DataType,
    span: Span,
    expected: DataType,
    field_name: &str,
) -> Result<(), PtxParseError> {
    if data_type == expected {
        Ok(())
    } else {
        Err(unexpected_value(
            span,
            &[match expected {
                DataType::B32 => ".b32",
                DataType::B64 => ".b64",
            }],
            format!("{field_name} expects {expected:?} but found {data_type:?}"),
        ))
    }
}

fn parse_element_type(value: u32, span: Span) -> Result<ElementType, PtxParseError> {
    match value {
        0 => Ok(ElementType::U8),
        1 => Ok(ElementType::U16),
        2 => Ok(ElementType::U32),
        3 => Ok(ElementType::S32),
        4 => Ok(ElementType::U64),
        5 => Ok(ElementType::S64),
        6 => Ok(ElementType::F16),
        7 => Ok(ElementType::F32),
        8 => Ok(ElementType::F64),
        9 => Ok(ElementType::Bf16),
        10 => Ok(ElementType::F32Ftz),
        11 => Ok(ElementType::Tf32),
        12 => Ok(ElementType::Tf32Ftz),
        13 => Ok(ElementType::B4x16),
        14 => Ok(ElementType::B4x16P64),
        15 => Ok(ElementType::B6x16P32),
        other => Err(unexpected_value(
            span,
            &[
                "0 (u8)",
                "1 (u16)",
                "2 (u32)",
                "3 (s32)",
                "4 (u64)",
                "5 (s64)",
                "6 (f16)",
                "7 (f32)",
                "8 (f64)",
                "9 (bf16)",
                "10 (f32ftz)",
                "11 (tf32)",
                "12 (tf32ftz)",
                "13 (b4x16)",
                "14 (b4x16_p64)",
                "15 (b6x16_p32)",
            ],
            other.to_string(),
        )),
    }
}

fn parse_interleave_layout(value: u32, span: Span) -> Result<InterleaveLayout, PtxParseError> {
    match value {
        0 => Ok(InterleaveLayout::None),
        1 => Ok(InterleaveLayout::Interleave16B),
        2 => Ok(InterleaveLayout::Interleave32B),
        other => Err(unexpected_value(
            span,
            &["0 (none)", "1 (interleave16B)", "2 (interleave32B)"],
            other.to_string(),
        )),
    }
}

fn parse_swizzle_mode(value: u32, span: Span) -> Result<SwizzleMode, PtxParseError> {
    match value {
        0 => Ok(SwizzleMode::None),
        1 => Ok(SwizzleMode::Swizzle32B),
        2 => Ok(SwizzleMode::Swizzle64B),
        3 => Ok(SwizzleMode::Swizzle128B),
        4 => Ok(SwizzleMode::Swizzle96B),
        other => Err(unexpected_value(
            span,
            &[
                "0 (none)",
                "1 (swizzle32B)",
                "2 (swizzle64B)",
                "3 (swizzle128B)",
                "4 (swizzle96B)",
            ],
            other.to_string(),
        )),
    }
}

fn parse_swizzle_atomicity(value: u32, span: Span) -> Result<SwizzleAtomicity, PtxParseError> {
    match value {
        0 => Ok(SwizzleAtomicity::Bytes16),
        1 => Ok(SwizzleAtomicity::Bytes32),
        2 => Ok(SwizzleAtomicity::Bytes32With8ByteFlip),
        3 => Ok(SwizzleAtomicity::Bytes64),
        other => Err(unexpected_value(
            span,
            &["0 (16B)", "1 (32B)", "2 (32B with 8B flip)", "3 (64B)"],
            other.to_string(),
        )),
    }
}

fn parse_fill_mode(value: u32, span: Span) -> Result<FillMode, PtxParseError> {
    match value {
        0 => Ok(FillMode::Zero),
        1 => Ok(FillMode::OobNan),
        other => Err(unexpected_value(
            span,
            &["0 (zero)", "1 (oob_nan)"],
            other.to_string(),
        )),
    }
}

impl PtxParser for Mode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "tile" => Ok(Mode::Tile),
            other => Err(unexpected_value(span, &[".tile"], format!(".{other}"))),
        }
    }
}

impl PtxParser for StateSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "global" => Ok(StateSpace::Global),
            "shared" => {
                stream.expect_double_colon()?;
                let (identifier, identifier_span) = stream.expect_identifier()?;
                if identifier == "cta" {
                    Ok(StateSpace::SharedCta)
                } else {
                    Err(unexpected_value(identifier_span, &["cta"], identifier))
                }
            }
            other => Err(unexpected_value(
                span,
                &[".global", ".shared::cta"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for ObjectSize {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b1024" => Ok(ObjectSize::B1024),
            other => Err(unexpected_value(span, &[".b1024"], format!(".{other}"))),
        }
    }
}

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        parse_data_type_with_span(stream).map(|(data_type, _)| data_type)
    }
}

impl PtxParser for TensormapOpcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "tensormap" {
            return Err(unexpected_value(span, &["tensormap"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_directive()?;
        if modifier != "replace" {
            return Err(unexpected_value(
                modifier_span,
                &[".replace"],
                format!(".{modifier}"),
            ));
        }

        let mode = Mode::parse(stream)?;
        let (field_directive, field_span) = stream.expect_directive()?;

        match field_directive.as_str() {
            "global_address" | "rank" => parse_field1(stream, mode, field_directive, field_span),
            "box_dim" | "global_dim" | "global_stride" | "element_stride" => {
                parse_field2(stream, mode, field_directive, field_span)
            }
            "elemtype" | "interleave_layout" | "swizzle_mode" | "swizzle_atomicity"
            | "fill_mode" => parse_field3(stream, mode, field_directive, field_span),
            other => Err(unexpected_value(
                field_span,
                &[
                    ".global_address",
                    ".rank",
                    ".box_dim",
                    ".global_dim",
                    ".global_stride",
                    ".element_stride",
                    ".elemtype",
                    ".interleave_layout",
                    ".swizzle_mode",
                    ".swizzle_atomicity",
                    ".fill_mode",
                ],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_field1(
    stream: &mut PtxTokenStream,
    mode: Mode,
    field_name: String,
    field_span: Span,
) -> Result<TensormapOpcode, PtxParseError> {
    let state_space = parse_optional_state_space(stream)?;
    let object_size = ObjectSize::parse(stream)?;
    let (data_type, data_type_span) = parse_data_type_with_span(stream)?;

    match field_name.as_str() {
        "global_address" => expect_data_type(
            data_type,
            data_type_span.clone(),
            DataType::B64,
            ".global_address",
        )?,
        "rank" => expect_data_type(data_type, data_type_span.clone(), DataType::B32, ".rank")?,
        _ => return Err(unexpected_value(field_span, &["field1"], field_name)),
    }

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let register = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    let field = match field_name.as_str() {
        "global_address" => Field1Field::GlobalAddress(register),
        "rank" => Field1Field::Rank(register),
        _ => unreachable!(),
    };

    Ok(TensormapOpcode::Field1(Field1 {
        mode,
        state_space,
        object_size,
        data_type,
        address,
        field,
    }))
}

fn parse_field2(
    stream: &mut PtxTokenStream,
    mode: Mode,
    field_name: String,
    field_span: Span,
) -> Result<TensormapOpcode, PtxParseError> {
    let state_space = parse_optional_state_space(stream)?;
    let object_size = ObjectSize::parse(stream)?;
    let (data_type, data_type_span) = parse_data_type_with_span(stream)?;

    match field_name.as_str() {
        "global_stride" => expect_data_type(
            data_type,
            data_type_span.clone(),
            DataType::B64,
            ".global_stride",
        )?,
        "box_dim" | "global_dim" | "element_stride" => expect_data_type(
            data_type,
            data_type_span.clone(),
            DataType::B32,
            field_name.as_str(),
        )?,
        _ => {
            return Err(unexpected_value(
                field_span,
                &[
                    ".box_dim",
                    ".global_dim",
                    ".global_stride",
                    ".element_stride",
                ],
                format!(".{field_name}"),
            ));
        }
    }

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (ordinal, _) = parse_u32_literal(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let register = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    let field = match field_name.as_str() {
        "box_dim" => Field2Field::BoxDim(register),
        "global_dim" => Field2Field::GlobalDim(register),
        "global_stride" => Field2Field::GlobalStride(register),
        "element_stride" => Field2Field::ElementStride(register),
        _ => unreachable!(),
    };

    Ok(TensormapOpcode::Field2(Field2 {
        mode,
        state_space,
        object_size,
        data_type,
        address,
        ordinal,
        field,
    }))
}

fn parse_field3(
    stream: &mut PtxTokenStream,
    mode: Mode,
    field_name: String,
    field_span: Span,
) -> Result<TensormapOpcode, PtxParseError> {
    let state_space = parse_optional_state_space(stream)?;
    let object_size = ObjectSize::parse(stream)?;
    let (data_type, data_type_span) = parse_data_type_with_span(stream)?;

    expect_data_type(
        data_type,
        data_type_span,
        DataType::B32,
        field_name.as_str(),
    )?;

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let (value, value_span) = parse_u32_literal(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    let field = match field_name.as_str() {
        "elemtype" => Field3Field::ElemType(parse_element_type(value, value_span)?),
        "interleave_layout" => {
            Field3Field::InterleaveLayout(parse_interleave_layout(value, value_span)?)
        }
        "swizzle_mode" => Field3Field::SwizzleMode(parse_swizzle_mode(value, value_span)?),
        "swizzle_atomicity" => {
            Field3Field::SwizzleAtomicity(parse_swizzle_atomicity(value, value_span)?)
        }
        "fill_mode" => Field3Field::FillMode(parse_fill_mode(value, value_span)?),
        _ => {
            return Err(unexpected_value(
                field_span,
                &[
                    ".elemtype",
                    ".interleave_layout",
                    ".swizzle_mode",
                    ".swizzle_atomicity",
                    ".fill_mode",
                ],
                format!(".{field_name}"),
            ));
        }
    };

    Ok(TensormapOpcode::Field3(Field3 {
        mode,
        state_space,
        object_size,
        data_type,
        address,
        field,
    }))
}
