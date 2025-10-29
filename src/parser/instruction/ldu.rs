use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::ldu::*},
};

enum VectorWidth {
    V2,
    V4,
}

impl VectorWidth {
    fn len(&self) -> usize {
        match self {
            VectorWidth::V2 => 2,
            VectorWidth::V4 => 4,
        }
    }
}

fn parse_optional_state_space(
    stream: &mut PtxTokenStream,
) -> Result<Option<StateSpace>, PtxParseError> {
    if stream.check(|token| directive_matches(token, "global")) {
        let _ = stream.consume()?;
        Ok(Some(StateSpace::Global))
    } else {
        Ok(None)
    }
}

fn parse_optional_vector_width(
    stream: &mut PtxTokenStream,
) -> Result<Option<VectorWidth>, PtxParseError> {
    if stream.check(|token| directive_matches(token, "v2")) {
        let _ = stream.consume()?;
        Ok(Some(VectorWidth::V2))
    } else if stream.check(|token| directive_matches(token, "v4")) {
        let _ = stream.consume()?;
        Ok(Some(VectorWidth::V4))
    } else {
        Ok(None)
    }
}

fn parse_data_type(
    stream: &mut PtxTokenStream,
) -> Result<crate::r#type::instruction::ldu::DataType, PtxParseError> {
    let (modifier, span) = stream.expect_directive()?;
    let data_type = match modifier.as_str() {
        "b8" => crate::r#type::instruction::ldu::DataType::B8,
        "b16" => crate::r#type::instruction::ldu::DataType::B16,
        "b32" => crate::r#type::instruction::ldu::DataType::B32,
        "b64" => crate::r#type::instruction::ldu::DataType::B64,
        "b128" => crate::r#type::instruction::ldu::DataType::B128,
        "u8" => crate::r#type::instruction::ldu::DataType::U8,
        "u16" => crate::r#type::instruction::ldu::DataType::U16,
        "u32" => crate::r#type::instruction::ldu::DataType::U32,
        "u64" => crate::r#type::instruction::ldu::DataType::U64,
        "s8" => crate::r#type::instruction::ldu::DataType::S8,
        "s16" => crate::r#type::instruction::ldu::DataType::S16,
        "s32" => crate::r#type::instruction::ldu::DataType::S32,
        "s64" => crate::r#type::instruction::ldu::DataType::S64,
        "f32" => crate::r#type::instruction::ldu::DataType::F32,
        "f64" => crate::r#type::instruction::ldu::DataType::F64,
        other => {
            return Err(unexpected_value(
                span,
                &[
                    ".b8", ".b16", ".b32", ".b64", ".b128", ".u8", ".u16", ".u32", ".u64", ".s8",
                    ".s16", ".s32", ".s64", ".f32", ".f64",
                ],
                format!(".{other}"),
            ));
        }
    };
    Ok(data_type)
}

fn parse_scalar_destination(stream: &mut PtxTokenStream) -> Result<RegisterOperand, PtxParseError> {
    let (name, _) = crate::parser::common::parse_register_name(stream)?;
    Ok(RegisterOperand::Single(name))
}

fn parse_vector_destination(
    stream: &mut PtxTokenStream,
    width: VectorWidth,
) -> Result<VectorDestination, PtxParseError> {
    let (_, span_ref) = stream.expect(&PtxToken::LBrace)?;
    let mut span = span_ref.clone();
    let mut registers = Vec::with_capacity(width.len());

    for index in 0..width.len() {
        let (name, register_span) = crate::parser::common::parse_register_name(stream)?;
        registers.push(RegisterOperand::Single(name));
        span.end = register_span.end;

        if index + 1 < width.len() {
            let (_, comma_span) = stream.expect(&PtxToken::Comma)?;
            span.end = comma_span.end;
        }
    }

    let (_, closing_span) = stream.expect(&PtxToken::RBrace)?;
    span.end = closing_span.end;

    match width {
        VectorWidth::V2 => {
            let array: [RegisterOperand; 2] = registers
                .try_into()
                .map_err(|_| invalid_literal(span.clone(), "expected 2 destination registers"))?;
            Ok(VectorDestination::V2(array))
        }
        VectorWidth::V4 => {
            let array: [RegisterOperand; 4] = registers
                .try_into()
                .map_err(|_| invalid_literal(span.clone(), "expected 4 destination registers"))?;
            Ok(VectorDestination::V4(array))
        }
    }
}

impl PtxParser for Ldu {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "ldu")?;

        let state_space = parse_optional_state_space(stream)?;
        let vector_width = parse_optional_vector_width(stream)?;
        let data_type = parse_data_type(stream)?;

        let instruction = match vector_width {
            Some(width) => {
                let destination = parse_vector_destination(stream, width)?;
                stream.expect(&PtxToken::Comma)?;
                let address = AddressOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ldu::Vector(Vector {
                    state_space,
                    data_type,
                    destination,
                    address,
                })
            }
            None => {
                let destination = parse_scalar_destination(stream)?;
                stream.expect(&PtxToken::Comma)?;
                let address = AddressOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ldu::Scalar(Scalar {
                    state_space,
                    data_type,
                    destination,
                    address,
                })
            }
        };

        Ok(instruction)
    }
}
