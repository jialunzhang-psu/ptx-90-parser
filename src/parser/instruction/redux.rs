use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::redux::*},
};

impl PtxParser for crate::r#type::instruction::redux::DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "u32" => Ok(crate::r#type::instruction::redux::DataType::U32),
            "s32" => Ok(crate::r#type::instruction::redux::DataType::S32),
            other => Err(unexpected_value(
                span,
                &[".u32", ".s32"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_integer(
    stream: &mut PtxTokenStream,
    operator: IntegerOperator,
) -> Result<Redux, PtxParseError> {
    let data_type = crate::r#type::instruction::redux::DataType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let member_mask = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Redux::Integer(Integer {
        operator,
        data_type,
        destination,
        source,
        member_mask,
    }))
}

fn parse_bitwise(
    stream: &mut PtxTokenStream,
    operator: BitwiseOperator,
) -> Result<Redux, PtxParseError> {
    expect_directive_value(stream, "b32")?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let member_mask = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Redux::Bitwise(Bitwise {
        operator,
        destination,
        source,
        member_mask,
    }))
}

fn parse_float(
    stream: &mut PtxTokenStream,
    operator: FloatOperator,
) -> Result<Redux, PtxParseError> {
    let mut absolute = false;
    let mut propagate_nan = false;

    loop {
        match peek_directive(stream)? {
            Some((value, span)) if value == "abs" => {
                if absolute {
                    return Err(unexpected_value(
                        span,
                        &[".NaN", ".f32"],
                        ".abs".to_string(),
                    ));
                }
                stream.consume()?;
                absolute = true;
            }
            Some((value, span)) if value == "NaN" => {
                if propagate_nan {
                    return Err(unexpected_value(span, &[".f32"], ".NaN".to_string()));
                }
                stream.consume()?;
                propagate_nan = true;
            }
            _ => break,
        }
    }

    expect_directive_value(stream, "f32")?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let source = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let member_mask = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Redux::Float(Float {
        operator,
        absolute,
        propagate_nan,
        destination,
        source,
        member_mask,
    }))
}

impl PtxParser for Redux {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "redux" {
            return Err(unexpected_value(span, &["redux"], opcode));
        }

        expect_directive_value(stream, "sync")?;
        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "add" => parse_integer(stream, IntegerOperator::Add),
            "min" => parse_min_or_max(stream, true),
            "max" => parse_min_or_max(stream, false),
            "and" => parse_bitwise(stream, BitwiseOperator::And),
            "or" => parse_bitwise(stream, BitwiseOperator::Or),
            "xor" => parse_bitwise(stream, BitwiseOperator::Xor),
            other => Err(unexpected_value(
                span,
                &[".add", ".min", ".max", ".and", ".or", ".xor"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_min_or_max(stream: &mut PtxTokenStream, is_min: bool) -> Result<Redux, PtxParseError> {
    match peek_directive(stream)? {
        Some((next, _)) if next == "u32" || next == "s32" => parse_integer(
            stream,
            if is_min {
                IntegerOperator::Min
            } else {
                IntegerOperator::Max
            },
        ),
        Some((next, _)) if next == "abs" || next == "NaN" || next == "f32" => parse_float(
            stream,
            if is_min {
                FloatOperator::Min
            } else {
                FloatOperator::Max
            },
        ),
        Some((next, next_span)) => Err(unexpected_value(
            next_span,
            &[".u32", ".s32", ".abs", ".NaN", ".f32"],
            format!(".{next}"),
        )),
        None => {
            let (token, token_span) = stream.peek()?;
            Err(unexpected_value(
                token_span.clone(),
                &[".u32", ".s32", ".abs", ".NaN", ".f32"],
                format!("{token:?}"),
            ))
        }
    }
}
