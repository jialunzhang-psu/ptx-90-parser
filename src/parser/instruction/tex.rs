use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::tex::*},
};

enum TexKind {
    Regular,
    Base,
    Level,
    Gradient,
}

enum VectorLayout {
    Vector4(Vector4DataType),
    Vector2F16x2,
}

enum OffsetShape {
    Scalar,
    Pair,
    Quad,
}

enum OptionalOperand {
    Offset(Offset),
    Depth(RegisterOperand),
}

fn parse_modifier(stream: &mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    if stream.check(|token| matches!(token, PtxToken::Directive(_))) {
        stream.expect_directive()
    } else if let Some((_, dot_span)) = stream.consume_if(|token| matches!(token, PtxToken::Dot)) {
        let mut span = dot_span.clone();
        let (next_token, next_span) = stream.peek()?;
        match next_token {
            PtxToken::Identifier(_) => {
                let (identifier, ident_span) = stream.expect_identifier()?;
                span.end = ident_span.end;
                Ok((identifier, span))
            }
            PtxToken::DecimalInteger(value) => {
                let mut modifier = value.clone();
                span.end = next_span.end;
                stream.consume()?;

                if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
                    let (suffix, suffix_span) = stream.expect_identifier()?;
                    modifier.push_str(&suffix);
                    span.end = suffix_span.end;
                }

                Ok((modifier, span))
            }
            token => Err(unexpected_value(
                next_span.clone(),
                &["identifier", "decimal digits"],
                format!("{token:?}"),
            )),
        }
    } else {
        let (token, span) = stream.peek()?;
        Err(unexpected_value(
            span.clone(),
            &["directive"],
            format!("{token:?}"),
        ))
    }
}

fn geometry_expected() -> [&'static str; 9] {
    [
        ".1d", ".2d", ".3d", ".a1d", ".a2d", ".cube", ".acube", ".2dms", ".a2dms",
    ]
}

fn parse_geometry_mod(value: &str, span: Span) -> Result<Geometry, PtxParseError> {
    match value {
        "1d" => Ok(Geometry::OneD),
        "2d" => Ok(Geometry::TwoD),
        "3d" => Ok(Geometry::ThreeD),
        "a1d" => Ok(Geometry::Array1D),
        "a2d" => Ok(Geometry::Array2D),
        "cube" => Ok(Geometry::Cube),
        "acube" => Ok(Geometry::ArrayCube),
        "2dms" => Ok(Geometry::TwoDMultisample),
        "a2dms" => Ok(Geometry::Array2DMultisample),
        other => Err(unexpected_value(
            span,
            &geometry_expected(),
            format!(".{other}"),
        )),
    }
}

fn parse_vector_layout(stream: &mut PtxTokenStream) -> Result<VectorLayout, PtxParseError> {
    let (modifier, span) = parse_modifier(stream)?;
    match modifier.as_str() {
        "v4" => {
            let (dtype, dtype_span) = parse_modifier(stream)?;
            let data_type = match dtype.as_str() {
                "u32" => Vector4DataType::U32,
                "s32" => Vector4DataType::S32,
                "f16" => Vector4DataType::F16,
                "f32" => Vector4DataType::F32,
                other => {
                    return Err(unexpected_value(
                        dtype_span,
                        &[".u32", ".s32", ".f16", ".f32"],
                        format!(".{other}"),
                    ));
                }
            };
            Ok(VectorLayout::Vector4(data_type))
        }
        "v2" => {
            let (dtype, dtype_span) = parse_modifier(stream)?;
            if dtype == "f16x2" {
                Ok(VectorLayout::Vector2F16x2)
            } else {
                Err(unexpected_value(
                    dtype_span,
                    &[".f16x2"],
                    format!(".{dtype}"),
                ))
            }
        }
        other => Err(unexpected_value(span, &[".v4", ".v2"], format!(".{other}"))),
    }
}

fn parse_coordinate_type(
    stream: &mut PtxTokenStream,
) -> Result<(CoordinateType, Span), PtxParseError> {
    let (modifier, span) = parse_modifier(stream)?;
    match modifier.as_str() {
        "s32" => Ok((CoordinateType::S32, span)),
        "f32" => Ok((CoordinateType::F32, span)),
        other => Err(unexpected_value(
            span,
            &[".s32", ".f32"],
            format!(".{other}"),
        )),
    }
}

fn validate_coordinate_type(
    geometry: Geometry,
    coordinate_type: CoordinateType,
    span: &Span,
) -> Result<(), PtxParseError> {
    match geometry {
        Geometry::Cube | Geometry::ArrayCube => {
            if coordinate_type != CoordinateType::F32 {
                return Err(unexpected_value(
                    span.clone(),
                    &[".f32"],
                    format!(
                        ".{}",
                        match coordinate_type {
                            CoordinateType::S32 => "s32",
                            CoordinateType::F32 => "f32",
                        }
                    ),
                ));
            }
        }
        Geometry::TwoDMultisample | Geometry::Array2DMultisample => {
            if coordinate_type != CoordinateType::S32 {
                return Err(unexpected_value(
                    span.clone(),
                    &[".s32"],
                    format!(
                        ".{}",
                        match coordinate_type {
                            CoordinateType::S32 => "s32",
                            CoordinateType::F32 => "f32",
                        }
                    ),
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn parse_sampler_and_coordinates(
    stream: &mut PtxTokenStream,
) -> Result<(Option<SamplerOperand>, RegisterOperand), PtxParseError> {
    let position = stream.position();
    if let Ok(candidate) = SamplerOperand::parse(stream) {
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            if let Ok(coordinates) = RegisterOperand::parse(stream) {
                return Ok((Some(candidate), coordinates));
            }
        }
        stream.set_position(position);
    } else {
        stream.set_position(position);
    }

    let coordinates = RegisterOperand::parse(stream)?;
    Ok((None, coordinates))
}

fn offset_shape_for_geometry(geometry: Geometry) -> Option<OffsetShape> {
    match geometry {
        Geometry::OneD | Geometry::Array1D => Some(OffsetShape::Scalar),
        Geometry::TwoD
        | Geometry::Array2D
        | Geometry::TwoDMultisample
        | Geometry::Array2DMultisample => Some(OffsetShape::Pair),
        Geometry::ThreeD => Some(OffsetShape::Quad),
        Geometry::Cube | Geometry::ArrayCube => None,
    }
}

fn is_float_register(name: &str) -> bool {
    name.starts_with("%f")
}

fn classify_optional_operand(geometry: Geometry, operand: RegisterOperand) -> OptionalOperand {
    match operand {
        RegisterOperand::Vector2(values) => {
            OptionalOperand::Offset(Offset::Pair(RegisterOperand::Vector2(values)))
        }
        RegisterOperand::Vector4(values) => {
            OptionalOperand::Offset(Offset::Quad(RegisterOperand::Vector4(values)))
        }
        RegisterOperand::Single(name) => {
            if matches!(
                offset_shape_for_geometry(geometry),
                Some(OffsetShape::Scalar)
            ) && !is_float_register(&name)
            {
                OptionalOperand::Offset(Offset::Scalar(RegisterOperand::Single(name)))
            } else {
                OptionalOperand::Depth(RegisterOperand::Single(name))
            }
        }
    }
}

fn parse_optional_offset_and_depth(
    stream: &mut PtxTokenStream,
    geometry: Geometry,
) -> Result<(Option<Offset>, Option<RegisterOperand>), PtxParseError> {
    let mut offset = None;
    let mut depth_compare = None;

    if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        let operand = RegisterOperand::parse(stream)?;
        match classify_optional_operand(geometry, operand) {
            OptionalOperand::Offset(value) => {
                offset = Some(value);
                if stream
                    .consume_if(|token| matches!(token, PtxToken::Comma))
                    .is_some()
                {
                    depth_compare = Some(RegisterOperand::parse(stream)?);
                }
            }
            OptionalOperand::Depth(value) => {
                depth_compare = Some(value);
            }
        }
    }

    Ok((offset, depth_compare))
}

fn to_gradient_vector(operand: RegisterOperand) -> GradientVector {
    match operand {
        RegisterOperand::Single(name) => GradientVector::Scalar(RegisterOperand::Single(name)),
        RegisterOperand::Vector2(values) => GradientVector::Pair(RegisterOperand::Vector2(values)),
        RegisterOperand::Vector4(values) => GradientVector::Quad(RegisterOperand::Vector4(values)),
    }
}

impl PtxParser for TextureOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            Ok(TextureOperand::Symbol(VariableSymbol::parse(stream)?))
        } else if stream.check(|token| matches!(token, PtxToken::Register(_))) {
            Ok(TextureOperand::Register(RegisterOperand::parse(stream)?))
        } else {
            let (token, span) = stream.peek()?;
            Err(unexpected_value(
                span.clone(),
                &["texture identifier", "register"],
                format!("{token:?}"),
            ))
        }
    }
}

impl PtxParser for SamplerOperand {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
            Ok(SamplerOperand::Symbol(VariableSymbol::parse(stream)?))
        } else if stream.check(|token| matches!(token, PtxToken::Register(_))) {
            Ok(SamplerOperand::Register(RegisterOperand::parse(stream)?))
        } else {
            let (token, span) = stream.peek()?;
            Err(unexpected_value(
                span.clone(),
                &["sampler identifier", "register"],
                format!("{token:?}"),
            ))
        }
    }
}

impl PtxParser for Tex {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "tex")?;

        let (modifier, span) = parse_modifier(stream)?;
        let (kind, geometry) = match modifier.as_str() {
            "base" => {
                let (geom, geom_span) = parse_modifier(stream)?;
                (TexKind::Base, parse_geometry_mod(&geom, geom_span.clone())?)
            }
            "level" => {
                let (geom, geom_span) = parse_modifier(stream)?;
                (
                    TexKind::Level,
                    parse_geometry_mod(&geom, geom_span.clone())?,
                )
            }
            "grad" => {
                let (geom, geom_span) = parse_modifier(stream)?;
                (
                    TexKind::Gradient,
                    parse_geometry_mod(&geom, geom_span.clone())?,
                )
            }
            other => (TexKind::Regular, parse_geometry_mod(other, span.clone())?),
        };

        let layout = parse_vector_layout(stream)?;
        let (coordinate_type, coordinate_span) = parse_coordinate_type(stream)?;
        validate_coordinate_type(geometry, coordinate_type, &coordinate_span)?;

        let destination = RegisterOperand::parse(stream)?;
        let predicate = if stream
            .consume_if(|token| matches!(token, PtxToken::Pipe))
            .is_some()
        {
            Some(PredicateRegister::parse(stream)?)
        } else {
            None
        };

        stream.expect(&PtxToken::Comma)?;
        stream.expect(&PtxToken::LBracket)?;

        let texture = TextureOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let (sampler, coordinates) = parse_sampler_and_coordinates(stream)?;

        stream.expect(&PtxToken::RBracket)?;

        let mut level_of_detail = None;
        let mut gradients = None;

        match kind {
            TexKind::Level => {
                stream.expect(&PtxToken::Comma)?;
                let operand = RegisterOperand::parse(stream)?;
                level_of_detail = Some(match coordinate_type {
                    CoordinateType::S32 => LevelOfDetail::S32(operand),
                    CoordinateType::F32 => LevelOfDetail::F32(operand),
                });
            }
            TexKind::Gradient => {
                stream.expect(&PtxToken::Comma)?;
                let dpdx = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let dpdy = RegisterOperand::parse(stream)?;
                gradients = Some(Gradients {
                    dpdx: to_gradient_vector(dpdx),
                    dpdy: to_gradient_vector(dpdy),
                });
            }
            TexKind::Regular | TexKind::Base => {}
        }

        let (offset, depth_compare) = parse_optional_offset_and_depth(stream, geometry)?;

        stream.expect(&PtxToken::Semicolon)?;

        match (kind, layout) {
            (TexKind::Regular, VectorLayout::Vector4(data_type)) => {
                if let Some(sampler) = sampler {
                    Ok(Tex::Vector4ExplicitSampler(Vector4ExplicitSampler {
                        geometry,
                        data_type,
                        coordinate_type,
                        destination,
                        predicate,
                        texture,
                        sampler,
                        coordinates,
                        offset,
                        depth_compare,
                    }))
                } else {
                    Ok(Tex::Vector4ImplicitSampler(Vector4ImplicitSampler {
                        geometry,
                        data_type,
                        coordinate_type,
                        destination,
                        predicate,
                        texture,
                        coordinates,
                        offset,
                        depth_compare,
                    }))
                }
            }
            (TexKind::Regular, VectorLayout::Vector2F16x2) => {
                if let Some(sampler) = sampler {
                    Ok(Tex::Vector2F16x2ExplicitSampler(
                        Vector2F16x2ExplicitSampler {
                            geometry,
                            coordinate_type,
                            destination,
                            predicate,
                            texture,
                            sampler,
                            coordinates,
                            offset,
                            depth_compare,
                        },
                    ))
                } else {
                    Ok(Tex::Vector2F16x2ImplicitSampler(
                        Vector2F16x2ImplicitSampler {
                            geometry,
                            coordinate_type,
                            destination,
                            predicate,
                            texture,
                            coordinates,
                            offset,
                            depth_compare,
                        },
                    ))
                }
            }
            (TexKind::Base, VectorLayout::Vector4(data_type)) => {
                Ok(Tex::Vector4MipBase(Vector4MipBase {
                    geometry,
                    data_type,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    offset,
                    depth_compare,
                }))
            }
            (TexKind::Base, VectorLayout::Vector2F16x2) => {
                Ok(Tex::Vector2F16x2MipBase(Vector2F16x2MipBase {
                    geometry,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    offset,
                    depth_compare,
                }))
            }
            (TexKind::Level, VectorLayout::Vector4(data_type)) => {
                Ok(Tex::Vector4MipLevel(Vector4MipLevel {
                    geometry,
                    data_type,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    level_of_detail: level_of_detail.expect("level of detail must be parsed"),
                    offset,
                    depth_compare,
                }))
            }
            (TexKind::Level, VectorLayout::Vector2F16x2) => {
                Ok(Tex::Vector2F16x2MipLevel(Vector2F16x2MipLevel {
                    geometry,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    level_of_detail: level_of_detail.expect("level of detail must be parsed"),
                    offset,
                    depth_compare,
                }))
            }
            (TexKind::Gradient, VectorLayout::Vector4(data_type)) => {
                Ok(Tex::Vector4MipGradient(Vector4MipGradient {
                    geometry,
                    data_type,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    gradients: gradients.expect("gradients must be parsed"),
                    offset,
                    depth_compare,
                }))
            }
            (TexKind::Gradient, VectorLayout::Vector2F16x2) => {
                Ok(Tex::Vector2F16x2MipGradient(Vector2F16x2MipGradient {
                    geometry,
                    coordinate_type,
                    destination,
                    predicate,
                    texture,
                    sampler,
                    coordinates,
                    gradients: gradients.expect("gradients must be parsed"),
                    offset,
                    depth_compare,
                }))
            }
        }
    }
}
