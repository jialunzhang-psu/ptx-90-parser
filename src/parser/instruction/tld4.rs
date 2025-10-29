use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::{PredicateRegister, RegisterOperand, VariableSymbol},
        instruction::tld4::*,
    },
};

enum GeometryKind {
    TwoD,
    Array2D,
    Cube,
    ArrayCube,
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

fn parse_component(stream: &mut PtxTokenStream) -> Result<Component, PtxParseError> {
    let (modifier, span) = parse_modifier(stream)?;
    match modifier.as_str() {
        "r" => Ok(Component::R),
        "g" => Ok(Component::G),
        "b" => Ok(Component::B),
        "a" => Ok(Component::A),
        other => Err(unexpected_value(
            span,
            &[".r", ".g", ".b", ".a"],
            format!(".{other}"),
        )),
    }
}

fn parse_geometry_kind(stream: &mut PtxTokenStream) -> Result<GeometryKind, PtxParseError> {
    let (modifier, span) = parse_modifier(stream)?;
    match modifier.as_str() {
        "2d" => Ok(GeometryKind::TwoD),
        "a2d" => Ok(GeometryKind::Array2D),
        "cube" => Ok(GeometryKind::Cube),
        "acube" => Ok(GeometryKind::ArrayCube),
        other => Err(unexpected_value(
            span,
            &[".2d", ".a2d", ".cube", ".acube"],
            format!(".{other}"),
        )),
    }
}

fn map_data_type(modifier: &str, span: Span) -> Result<DataType, PtxParseError> {
    match modifier {
        "u32" => Ok(DataType::U32),
        "s32" => Ok(DataType::S32),
        "f32" => Ok(DataType::F32),
        other => Err(unexpected_value(
            span,
            &[".u32", ".s32", ".f32"],
            format!(".{other}"),
        )),
    }
}

fn parse_data_type(stream: &mut PtxTokenStream) -> Result<DataType, PtxParseError> {
    let (modifier, span) = parse_modifier(stream)?;
    map_data_type(&modifier, span)
}

impl PtxParser for Destination {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let vector = RegisterOperand::parse(stream)?;
        let predicate = if stream
            .consume_if(|token| matches!(token, PtxToken::Pipe))
            .is_some()
        {
            Some(PredicateRegister::parse(stream)?)
        } else {
            None
        };

        Ok(Destination { vector, predicate })
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

impl PtxParser for Tld4 {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "tld4" {
            return Err(unexpected_value(span, &["tld4"], opcode));
        }

        let component = parse_component(stream)?;
        let geometry_kind = parse_geometry_kind(stream)?;
        let (vector, vector_span) = parse_modifier(stream)?;
        if vector != "v4" {
            return Err(unexpected_value(
                vector_span,
                &[".v4"],
                format!(".{vector}"),
            ));
        }

        let (modifier, span) = parse_modifier(stream)?;
        let data_type = if modifier == "dtype" {
            parse_data_type(stream)?
        } else {
            map_data_type(&modifier, span.clone())?
        };

        let destination = Destination::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        stream.expect(&PtxToken::LBracket)?;

        let texture = TextureOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;

        let position = stream.position();
        let mut sampler = None;
        let coordinates;

        match SamplerOperand::parse(stream) {
            Ok(candidate) => {
                if stream
                    .consume_if(|token| matches!(token, PtxToken::Comma))
                    .is_some()
                {
                    match RegisterOperand::parse(stream) {
                        Ok(coord) => {
                            sampler = Some(candidate);
                            coordinates = coord;
                        }
                        Err(_) => {
                            stream.set_position(position);
                            coordinates = RegisterOperand::parse(stream)?;
                        }
                    }
                } else {
                    stream.set_position(position);
                    coordinates = RegisterOperand::parse(stream)?;
                }
            }
            Err(_) => {
                stream.set_position(position);
                coordinates = RegisterOperand::parse(stream)?;
            }
        }

        stream.expect(&PtxToken::RBracket)?;

        let mut offset: Option<RegisterOperand> = None;
        let mut depth_compare: Option<RegisterOperand> = None;

        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            let operand = RegisterOperand::parse(stream)?;

            match geometry_kind {
                GeometryKind::TwoD | GeometryKind::Array2D => match operand {
                    RegisterOperand::Vector2(_) | RegisterOperand::Vector4(_) => {
                        offset = Some(operand);

                        if stream
                            .consume_if(|token| matches!(token, PtxToken::Comma))
                            .is_some()
                        {
                            depth_compare = Some(RegisterOperand::parse(stream)?);
                        }
                    }
                    RegisterOperand::Single(_) => {
                        depth_compare = Some(operand);
                    }
                },
                GeometryKind::Cube | GeometryKind::ArrayCube => {
                    depth_compare = Some(operand);
                }
            }
        }

        stream.expect(&PtxToken::Semicolon)?;

        let geometry = match geometry_kind {
            GeometryKind::TwoD => Geometry::TwoD {
                coordinates,
                offset,
            },
            GeometryKind::Array2D => Geometry::Array2D {
                coordinates,
                offset,
            },
            GeometryKind::Cube => Geometry::Cube { coordinates },
            GeometryKind::ArrayCube => Geometry::ArrayCube { coordinates },
        };

        if let Some(sampler) = sampler {
            Ok(Tld4::Explicit(ExplicitSampler {
                component,
                geometry,
                data_type,
                destination,
                texture,
                sampler,
                depth_compare,
            }))
        } else {
            Ok(Tld4::Implicit(ImplicitSampler {
                component,
                geometry,
                data_type,
                destination,
                texture,
                depth_compare,
            }))
        }
    }
}
