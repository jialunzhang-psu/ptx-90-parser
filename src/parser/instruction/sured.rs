use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::sured::*},
};

enum AddressingKind {
    Byte,
    Sample,
}

enum CoordinatesData {
    Byte(ByteDataType),
    Sample(SampleDataType),
}

enum Geometry {
    OneD,
    TwoD,
    ThreeD,
}

enum Coordinates {
    OneD(Coordinate1d),
    TwoD(Coordinate2d),
    ThreeD(Coordinate3d),
}

fn parse_addressing_kind(stream: &mut PtxTokenStream) -> Result<AddressingKind, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "b" => Ok(AddressingKind::Byte),
        "p" => Ok(AddressingKind::Sample),
        other => Err(unexpected_value(span, &[".b", ".p"], format!(".{other}"))),
    }
}

fn parse_operator(stream: &mut PtxTokenStream) -> Result<Operator, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "add" => Ok(Operator::Add),
        "min" => Ok(Operator::Min),
        "max" => Ok(Operator::Max),
        "and" => Ok(Operator::And),
        "or" => Ok(Operator::Or),
        other => Err(unexpected_value(
            span,
            &[".add", ".min", ".max", ".and", ".or"],
            format!(".{other}"),
        )),
    }
}

fn parse_geometry(stream: &mut PtxTokenStream) -> Result<Geometry, PtxParseError> {
    if let Some((modifier, span)) = peek_directive(stream)? {
        stream.expect_directive()?;
        return match modifier.as_str() {
            "1d" => Ok(Geometry::OneD),
            "2d" => Ok(Geometry::TwoD),
            "3d" => Ok(Geometry::ThreeD),
            other => Err(unexpected_value(
                span,
                &[".1d", ".2d", ".3d"],
                format!(".{other}"),
            )),
        };
    }

    if stream.check(|token| matches!(token, PtxToken::Dot)) {
        let (_, dot_span) = stream.consume()?;
        let mut span = dot_span.clone();

        let (number_token, number_span) = stream.consume()?;
        let number_span = number_span.clone();
        let number = match number_token {
            PtxToken::DecimalInteger(value) => value.clone(),
            other => {
                return Err(unexpected_value(
                    number_span.clone(),
                    &["geometry specifier"],
                    format!("{other:?}"),
                ));
            }
        };

        let (suffix_token, suffix_span) = stream.consume()?;
        let suffix_span = suffix_span.clone();
        let suffix = match suffix_token {
            PtxToken::Identifier(value) => value.clone(),
            other => {
                return Err(unexpected_value(
                    suffix_span.clone(),
                    &["geometry suffix"],
                    format!("{other:?}"),
                ));
            }
        };

        span.end = suffix_span.end;

        let modifier = format!("{number}{suffix}");
        return match modifier.as_str() {
            "1d" => Ok(Geometry::OneD),
            "2d" => Ok(Geometry::TwoD),
            "3d" => Ok(Geometry::ThreeD),
            other => Err(unexpected_value(
                span,
                &[".1d", ".2d", ".3d"],
                format!(".{other}"),
            )),
        };
    }

    let (token, span) = stream.peek()?;
    Err(unexpected_value(
        span.clone(),
        &[".1d", ".2d", ".3d"],
        format!("{token:?}"),
    ))
}

fn parse_byte_data_type(stream: &mut PtxTokenStream) -> Result<ByteDataType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "u32" => Ok(ByteDataType::U32),
        "u64" => Ok(ByteDataType::U64),
        "s32" => Ok(ByteDataType::S32),
        "b32" => Ok(ByteDataType::B32),
        "s64" => Ok(ByteDataType::S64),
        other => Err(unexpected_value(
            span,
            &[".u32", ".u64", ".s32", ".b32", ".s64"],
            format!(".{other}"),
        )),
    }
}

fn parse_sample_data_type(stream: &mut PtxTokenStream) -> Result<SampleDataType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "b32" => Ok(SampleDataType::B32),
        "b64" => Ok(SampleDataType::B64),
        other => Err(unexpected_value(
            span,
            &[".b32", ".b64"],
            format!(".{other}"),
        )),
    }
}

fn parse_clamp(stream: &mut PtxTokenStream) -> Result<Clamp, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "trap" => Ok(Clamp::Trap),
        "clamp" => Ok(Clamp::Clamp),
        "zero" => Ok(Clamp::Zero),
        other => Err(unexpected_value(
            span,
            &[".trap", ".clamp", ".zero"],
            format!(".{other}"),
        )),
    }
}

fn parse_surface(stream: &mut PtxTokenStream) -> Result<Surface, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        Ok(Surface::Reference(VariableSymbol::parse(stream)?))
    } else if stream.check(|token| matches!(token, PtxToken::Register(_))) {
        Ok(Surface::Indirect(RegisterOperand::parse(stream)?))
    } else {
        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["surface identifier", "register"],
            format!("{token:?}"),
        ))
    }
}

fn parse_coordinate_component(
    stream: &mut PtxTokenStream,
) -> Result<RegisterOperand, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::LBrace)) {
        let (_, span) = stream.peek()?;
        return Err(unexpected_value(
            span.clone(),
            &["register operand"],
            "{".to_string(),
        ));
    }

    RegisterOperand::parse(stream)
}

fn parse_coordinate_components(
    stream: &mut PtxTokenStream,
    count: usize,
) -> Result<Vec<RegisterOperand>, PtxParseError> {
    let in_braces = stream
        .consume_if(|token| matches!(token, PtxToken::LBrace))
        .is_some();

    let mut components = Vec::with_capacity(count);
    for index in 0..count {
        components.push(parse_coordinate_component(stream)?);
        if index + 1 < count {
            stream.expect(&PtxToken::Comma)?;
        }
    }

    if in_braces {
        stream.expect(&PtxToken::RBrace)?;
    }

    Ok(components)
}

fn parse_coordinate_1d(stream: &mut PtxTokenStream) -> Result<Coordinate1d, PtxParseError> {
    let mut components = parse_coordinate_components(stream, 1)?;
    Ok(components.remove(0))
}

fn parse_coordinate_2d(stream: &mut PtxTokenStream) -> Result<Coordinate2d, PtxParseError> {
    let mut components = parse_coordinate_components(stream, 2)?;
    Ok(Coordinate2d {
        x: components.remove(0),
        y: components.remove(0),
    })
}

fn parse_coordinate_3d(stream: &mut PtxTokenStream) -> Result<Coordinate3d, PtxParseError> {
    let mut components = parse_coordinate_components(stream, 4)?;
    Ok(Coordinate3d {
        x: components.remove(0),
        y: components.remove(0),
        z: components.remove(0),
        w: components.remove(0),
    })
}

impl PtxParser for Sured {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "sured")?;

        let addressing_kind = parse_addressing_kind(stream)?;
        let operator = parse_operator(stream)?;
        let geometry = parse_geometry(stream)?;

        let data = match addressing_kind {
            AddressingKind::Byte => {
                let ty = parse_byte_data_type(stream)?;
                CoordinatesData::Byte(ty)
            }
            AddressingKind::Sample => {
                let ty = parse_sample_data_type(stream)?;
                CoordinatesData::Sample(ty)
            }
        };

        let clamp = parse_clamp(stream)?;

        stream.expect(&PtxToken::LBracket)?;
        let surface = parse_surface(stream)?;
        stream.expect(&PtxToken::Comma)?;

        let coordinates = match geometry {
            Geometry::OneD => Coordinates::OneD(parse_coordinate_1d(stream)?),
            Geometry::TwoD => Coordinates::TwoD(parse_coordinate_2d(stream)?),
            Geometry::ThreeD => Coordinates::ThreeD(parse_coordinate_3d(stream)?),
        };

        stream.expect(&PtxToken::RBracket)?;
        stream.expect(&PtxToken::Comma)?;
        let source = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;

        match (data, coordinates) {
            (CoordinatesData::Byte(data_type), Coordinates::OneD(coord)) => {
                Ok(Sured::Byte1d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
            (CoordinatesData::Byte(data_type), Coordinates::TwoD(coord)) => {
                Ok(Sured::Byte2d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
            (CoordinatesData::Byte(data_type), Coordinates::ThreeD(coord)) => {
                Ok(Sured::Byte3d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
            (CoordinatesData::Sample(data_type), Coordinates::OneD(coord)) => {
                Ok(Sured::Sample1d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
            (CoordinatesData::Sample(data_type), Coordinates::TwoD(coord)) => {
                Ok(Sured::Sample2d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
            (CoordinatesData::Sample(data_type), Coordinates::ThreeD(coord)) => {
                Ok(Sured::Sample3d(Reduction {
                    operator,
                    data_type,
                    clamp,
                    surface,
                    coordinates: coord,
                    source,
                }))
            }
        }
    }
}
