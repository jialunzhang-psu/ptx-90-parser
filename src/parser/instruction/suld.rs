use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::suld::*,
    },
};

fn parse_cache_operator(
    stream: &mut PtxTokenStream,
) -> Result<Option<CacheOperator>, PtxParseError> {
    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name)
                if matches!(name.as_str(), "ca" | "cg" | "cs" | "cv")
        )
    }) {
        let (modifier, _span) = stream.expect_directive()?;
        let cache = match modifier.as_str() {
            "ca" => CacheOperator::Ca,
            "cg" => CacheOperator::Cg,
            "cs" => CacheOperator::Cs,
            "cv" => CacheOperator::Cv,
            _ => unreachable!(),
        };
        Ok(Some(cache))
    } else {
        Ok(None)
    }
}

fn parse_vector(stream: &mut PtxTokenStream) -> Result<Vector, PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "v2")) {
        let _ = stream.expect_directive()?;
        Ok(Vector::V2)
    } else if stream.check(|token| matches!(token, PtxToken::Directive(name) if name == "v4")) {
        let _ = stream.expect_directive()?;
        Ok(Vector::V4)
    } else {
        Ok(Vector::None)
    }
}

fn parse_data_type(stream: &mut PtxTokenStream) -> Result<DataType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "b8" => Ok(DataType::B8),
        "b16" => Ok(DataType::B16),
        "b32" => Ok(DataType::B32),
        "b64" => Ok(DataType::B64),
        other => Err(unexpected_value(
            span,
            &[".b8", ".b16", ".b32", ".b64"],
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
        Ok(Surface::Register(RegisterOperand::parse(stream)?))
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

    let operand = RegisterOperand::parse(stream)?;
    Ok(operand)
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
    Ok(Coordinate1d {
        x: components.remove(0),
    })
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

fn parse_array_coordinate_1d(
    stream: &mut PtxTokenStream,
) -> Result<Array1dCoordinates, PtxParseError> {
    let mut components = parse_coordinate_components(stream, 2)?;
    Ok(Array1dCoordinates {
        index: components.remove(0),
        x: components.remove(0),
    })
}

fn parse_array_coordinate_2d(
    stream: &mut PtxTokenStream,
) -> Result<Array2dCoordinates, PtxParseError> {
    let mut components = parse_coordinate_components(stream, 4)?;
    Ok(Array2dCoordinates {
        index: components.remove(0),
        x: components.remove(0),
        y: components.remove(0),
        z: components.remove(0),
    })
}

enum Geometry {
    OneD,
    TwoD,
    ThreeD,
    Array1D,
    Array2D,
}

impl PtxParser for Suld {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "suld" {
            return Err(unexpected_value(span, &["suld"], opcode));
        }

        let (base, base_span) = stream.expect_directive()?;
        if base != "b" {
            return Err(unexpected_value(base_span, &[".b"], format!(".{base}")));
        }

        let (geom, geom_span) = stream.expect_directive()?;
        let geometry = match geom.as_str() {
            "1d" => Geometry::OneD,
            "2d" => Geometry::TwoD,
            "3d" => Geometry::ThreeD,
            "a1d" => Geometry::Array1D,
            "a2d" => Geometry::Array2D,
            other => {
                return Err(unexpected_value(
                    geom_span,
                    &[".1d", ".2d", ".3d", ".a1d", ".a2d"],
                    format!(".{other}"),
                ));
            }
        };

        let cache_operator = parse_cache_operator(stream)?;
        let vector = parse_vector(stream)?;
        let data_type = parse_data_type(stream)?;
        let clamp = parse_clamp(stream)?;

        let destination = RegisterOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        stream.expect(&PtxToken::LBracket)?;
        let surface = parse_surface(stream)?;
        stream.expect(&PtxToken::Comma)?;

        let result = match geometry {
            Geometry::OneD => {
                let coordinates = parse_coordinate_1d(stream)?;
                Suld::OneD(Descriptor {
                    cache_operator,
                    vector,
                    data_type,
                    clamp,
                    destination,
                    surface,
                    coordinates,
                })
            }
            Geometry::TwoD => {
                let coordinates = parse_coordinate_2d(stream)?;
                Suld::TwoD(Descriptor {
                    cache_operator,
                    vector,
                    data_type,
                    clamp,
                    destination,
                    surface,
                    coordinates,
                })
            }
            Geometry::ThreeD => {
                let coordinates = parse_coordinate_3d(stream)?;
                Suld::ThreeD(Descriptor {
                    cache_operator,
                    vector,
                    data_type,
                    clamp,
                    destination,
                    surface,
                    coordinates,
                })
            }
            Geometry::Array1D => {
                let coordinates = parse_array_coordinate_1d(stream)?;
                Suld::Array1D(Descriptor {
                    cache_operator,
                    vector,
                    data_type,
                    clamp,
                    destination,
                    surface,
                    coordinates,
                })
            }
            Geometry::Array2D => {
                let coordinates = parse_array_coordinate_2d(stream)?;
                Suld::Array2D(Descriptor {
                    cache_operator,
                    vector,
                    data_type,
                    clamp,
                    destination,
                    surface,
                    coordinates,
                })
            }
        };

        stream.expect(&PtxToken::RBracket)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(result)
    }
}
