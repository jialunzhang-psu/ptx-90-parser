use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::sust::*},
};

fn parse_cache_operator(
    stream: &mut PtxTokenStream,
) -> Result<Option<CacheOperator>, PtxParseError> {
    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name)
                if matches!(name.as_str(), "wb" | "cg" | "cs" | "wt")
        )
    }) {
        let (modifier, _span) = stream.expect_directive()?;
        let cache = match modifier.as_str() {
            "wb" => CacheOperator::Wb,
            "cg" => CacheOperator::Cg,
            "cs" => CacheOperator::Cs,
            "wt" => CacheOperator::Wt,
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

fn parse_component_type(stream: &mut PtxTokenStream) -> Result<ComponentType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "b8" => Ok(ComponentType::B8),
        "b16" => Ok(ComponentType::B16),
        "b32" => Ok(ComponentType::B32),
        "b64" => Ok(ComponentType::B64),
        other => Err(unexpected_value(
            span,
            &[".b8", ".b16", ".b32", ".b64"],
            format!(".{other}"),
        )),
    }
}

fn parse_formatted_component_type(
    stream: &mut PtxTokenStream,
) -> Result<FormattedComponentType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    match modifier.as_str() {
        "b32" => Ok(FormattedComponentType::B32),
        other => Err(unexpected_value(span, &[".b32"], format!(".{other}"))),
    }
}

fn parse_geometry_modifier(
    stream: &mut PtxTokenStream,
    expected: &[&str],
) -> Result<(String, Span), PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(_))) {
        return stream.expect_directive();
    }

    let position = stream.position();
    if let Some((_, dot_span)) = stream.consume_if(|token| matches!(token, PtxToken::Dot)) {
        match stream.consume() {
            Ok((token, span)) => {
                if let PtxToken::DecimalInteger(digits) = token {
                    let mut value = digits.clone();
                    let mut end = span.end;
                    if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
                        let (suffix, suffix_span) = stream.expect_identifier()?;
                        value.push_str(&suffix);
                        end = suffix_span.end;
                    }
                    return Ok((value, dot_span.start..end));
                }
                stream.set_position(position);
            }
            Err(err) => {
                stream.set_position(position);
                return Err(err);
            }
        }
    }

    let (token, span) = stream.peek()?;
    Err(unexpected_value(
        span.clone(),
        expected,
        format!("{token:?}"),
    ))
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

fn parse_block<TCoordinates, F>(
    stream: &mut PtxTokenStream,
    parse_coordinates: F,
) -> Result<Block<TCoordinates>, PtxParseError>
where
    F: FnOnce(&mut PtxTokenStream) -> Result<TCoordinates, PtxParseError>,
{
    let cache_operator = parse_cache_operator(stream)?;
    let vector = parse_vector(stream)?;
    let component_type = parse_component_type(stream)?;
    let clamp = parse_clamp(stream)?;

    stream.expect(&PtxToken::LBracket)?;
    let surface = parse_surface(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let coordinates = parse_coordinates(stream)?;
    stream.expect(&PtxToken::RBracket)?;
    stream.expect(&PtxToken::Comma)?;
    let value = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Block {
        cache_operator,
        vector,
        component_type,
        clamp,
        surface,
        coordinates,
        value,
    })
}

fn parse_formatted<TCoordinates, F>(
    stream: &mut PtxTokenStream,
    parse_coordinates: F,
) -> Result<Formatted<TCoordinates>, PtxParseError>
where
    F: FnOnce(&mut PtxTokenStream) -> Result<TCoordinates, PtxParseError>,
{
    let vector = parse_vector(stream)?;
    let component_type = parse_formatted_component_type(stream)?;
    let clamp = parse_clamp(stream)?;

    stream.expect(&PtxToken::LBracket)?;
    let surface = parse_surface(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let coordinates = parse_coordinates(stream)?;
    stream.expect(&PtxToken::RBracket)?;
    stream.expect(&PtxToken::Comma)?;
    let value = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Formatted {
        vector,
        component_type,
        clamp,
        surface,
        coordinates,
        value,
    })
}

impl PtxParser for Sust {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "sust" {
            return Err(unexpected_value(span, &["sust"], opcode));
        }

        let (mode, mode_span) = stream.expect_directive()?;
        match mode.as_str() {
            "b" => {
                let (geom, geom_span) =
                    parse_geometry_modifier(stream, &[".1d", ".2d", ".3d", ".a1d", ".a2d"])?;
                match geom.as_str() {
                    "1d" => Ok(Sust::Block1d(parse_block(stream, parse_coordinate_1d)?)),
                    "2d" => Ok(Sust::Block2d(parse_block(stream, parse_coordinate_2d)?)),
                    "3d" => Ok(Sust::Block3d(parse_block(stream, parse_coordinate_3d)?)),
                    "a1d" => Ok(Sust::BlockArray1d(parse_block(
                        stream,
                        parse_array_coordinate_1d,
                    )?)),
                    "a2d" => Ok(Sust::BlockArray2d(parse_block(
                        stream,
                        parse_array_coordinate_2d,
                    )?)),
                    other => Err(unexpected_value(
                        geom_span,
                        &[".1d", ".2d", ".3d", ".a1d", ".a2d"],
                        format!(".{other}"),
                    )),
                }
            }
            "p" => {
                let (geom, geom_span) = parse_geometry_modifier(stream, &[".1d", ".2d", ".3d"])?;
                match geom.as_str() {
                    "1d" => Ok(Sust::Formatted1d(parse_formatted(
                        stream,
                        parse_coordinate_1d,
                    )?)),
                    "2d" => Ok(Sust::Formatted2d(parse_formatted(
                        stream,
                        parse_coordinate_2d,
                    )?)),
                    "3d" => Ok(Sust::Formatted3d(parse_formatted(
                        stream,
                        parse_coordinate_3d,
                    )?)),
                    other => Err(unexpected_value(
                        geom_span,
                        &[".1d", ".2d", ".3d"],
                        format!(".{other}"),
                    )),
                }
            }
            other => Err(unexpected_value(
                mode_span,
                &[".b", ".p"],
                format!(".{other}"),
            )),
        }
    }
}
