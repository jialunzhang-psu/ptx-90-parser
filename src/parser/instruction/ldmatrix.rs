use crate::r#type::instruction::ldmatrix::DataType as LdmatrixDataType;
use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::ldmatrix::*},
};

impl PtxParser for Shape {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "m8n8" => Ok(Shape::M8N8),
            "m16n16" => Ok(Shape::M16N16),
            other => Err(unexpected_value(
                span,
                &[".m8n8", ".m16n16"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for Num {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "x1" => Ok(Num::X1),
            "x2" => Ok(Num::X2),
            "x4" => Ok(Num::X4),
            other => Err(unexpected_value(
                span,
                &[".x1", ".x2", ".x4"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for StateSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "shared" => {
                if stream.check(|token| matches!(token, PtxToken::Colon)) {
                    stream.expect_double_colon()?;
                    let (modifier, modifier_span) = stream.expect_identifier()?;
                    match modifier.as_str() {
                        "cta" => Ok(StateSpace::SharedCta),
                        other => Err(unexpected_value(modifier_span, &["cta"], other)),
                    }
                } else {
                    Ok(StateSpace::Shared)
                }
            }
            other => Err(unexpected_value(
                span,
                &[".shared", ".shared::cta"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for LdmatrixDataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b16" => Ok(LdmatrixDataType::B16),
            "b8" => Ok(LdmatrixDataType::B8),
            other => Err(unexpected_value(
                span,
                &[".b16", ".b8"],
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for DestinationFormat {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b8x16" => Ok(DestinationFormat::B8x16),
            other => Err(unexpected_value(span, &[".b8x16"], format!(".{other}"))),
        }
    }
}

impl PtxParser for SourceFormat {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b6x16_p32" => Ok(SourceFormat::B6x16P32),
            "b4x16_p64" => Ok(SourceFormat::B4x16P64),
            other => Err(unexpected_value(
                span,
                &[".b6x16_p32", ".b4x16_p64"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_optional_state_space(
    stream: &mut PtxTokenStream,
) -> Result<Option<StateSpace>, PtxParseError> {
    if stream.check(|token| directive_matches(token, "shared")) {
        StateSpace::parse(stream).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_standard_tail(
    stream: &mut PtxTokenStream,
    shape: Shape,
) -> Result<Ldmatrix, PtxParseError> {
    let num = Num::parse(stream)?;
    let trans = consume_directive_if(stream, "trans");
    let state_space = parse_optional_state_space(stream)?;
    let data_type = LdmatrixDataType::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Ldmatrix::Standard(Standard {
        shape,
        num,
        trans,
        state_space,
        data_type,
        destination,
        address,
    }))
}

fn parse_m8n16_tail(stream: &mut PtxTokenStream) -> Result<Ldmatrix, PtxParseError> {
    let num = Num::parse(stream)?;
    let state_space = parse_optional_state_space(stream)?;
    let destination_format = DestinationFormat::parse(stream)?;
    let source_format = SourceFormat::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Ldmatrix::M8N16(M8N16 {
        num,
        state_space,
        destination_format,
        source_format,
        destination,
        address,
    }))
}

fn parse_m16n16_special(stream: &mut PtxTokenStream) -> Result<Ldmatrix, PtxParseError> {
    let num = Num::parse(stream)?;
    expect_directive_value(stream, "trans")?;
    let state_space = parse_optional_state_space(stream)?;
    let destination_format = DestinationFormat::parse(stream)?;
    let source_format = SourceFormat::parse(stream)?;
    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Ldmatrix::M16N16(M16N16 {
        num,
        state_space,
        destination_format,
        source_format,
        destination,
        address,
    }))
}

impl PtxParser for Ldmatrix {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "ldmatrix")?;
        expect_directive_value(stream, "sync")?;
        expect_directive_value(stream, "aligned")?;

        let (modifier, span) = stream.expect_directive()?;
        match modifier.as_str() {
            "m8n8" => parse_standard_tail(stream, Shape::M8N8),
            "m8n16" => parse_m8n16_tail(stream),
            "m16n16" => {
                let checkpoint = stream.position();
                let num = Num::parse(stream)?;
                let trans = consume_directive_if(stream, "trans");
                let state_space = parse_optional_state_space(stream)?;
                match peek_directive(stream)? {
                    Some((next, _)) if next == "b16" || next == "b8" => {
                        let data_type = LdmatrixDataType::parse(stream)?;
                        let destination = RegisterOperand::parse(stream)?;
                        stream.expect(&PtxToken::Comma)?;
                        let address = AddressOperand::parse(stream)?;
                        stream.expect(&PtxToken::Semicolon)?;
                        Ok(Ldmatrix::Standard(Standard {
                            shape: Shape::M16N16,
                            num,
                            trans,
                            state_space,
                            data_type,
                            destination,
                            address,
                        }))
                    }
                    Some((next, _)) if next == "b8x16" => {
                        stream.set_position(checkpoint);
                        parse_m16n16_special(stream)
                    }
                    Some((next, span)) => Err(unexpected_value(
                        span,
                        &[".b16", ".b8", ".b8x16"],
                        format!(".{next}"),
                    )),
                    None => Err(PtxParseError {
                        kind: ParseErrorKind::UnexpectedEof,
                        span,
                    }),
                }
            }
            other => Err(unexpected_value(
                span,
                &[".m8n8", ".m8n16", ".m16n16"],
                format!(".{other}"),
            )),
        }
    }
}
