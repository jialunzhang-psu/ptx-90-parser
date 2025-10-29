use crate::{
    lexer::PtxToken,
    parser::{
        PtxParseError, PtxParser, PtxTokenStream, consume_directive_if, directive_matches,
        invalid_literal, unexpected_value,
    },
    r#type::{
        common::{AddressOperand, RegisterOperand},
        instruction::stmatrix::{DataType, Num, Shape, Source, StateSpace, Stmatrix},
    },
};

impl PtxParser for Shape {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "m8n8" => Ok(Shape::M8N8),
            "m16n8" => Ok(Shape::M16N8),
            other => Err(unexpected_value(
                span,
                &[".m8n8", ".m16n8"],
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

impl PtxParser for DataType {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "b16" => Ok(DataType::B16),
            "b8" => Ok(DataType::B8),
            other => Err(unexpected_value(
                span,
                &[".b16", ".b8"],
                format!(".{other}"),
            )),
        }
    }
}

fn parse_source(stream: &mut PtxTokenStream, num: Num) -> Result<Source, PtxParseError> {
    let (_, open_span) = stream.expect(&PtxToken::LBrace)?;
    let mut registers = Vec::new();

    while !stream.check(|token| matches!(token, PtxToken::RBrace)) {
        registers.push(RegisterOperand::parse(stream)?);
        if stream
            .consume_if(|token| matches!(token, PtxToken::Comma))
            .is_some()
        {
            continue;
        }
        break;
    }

    let (_, close_span) = stream.expect(&PtxToken::RBrace)?;

    let mut span = open_span.clone();
    span.end = close_span.end;

    let expected = match num {
        Num::X1 => 1,
        Num::X2 => 2,
        Num::X4 => 4,
    };

    if registers.len() != expected {
        return Err(invalid_literal(
            span,
            format!(
                "expected {expected} registers in source list, found {}",
                registers.len()
            ),
        ));
    }

    Ok(match num {
        Num::X1 => Source::X1(registers.into_iter().next().unwrap()),
        Num::X2 => {
            let mut iter = registers.into_iter();
            Source::X2([iter.next().unwrap(), iter.next().unwrap()])
        }
        Num::X4 => {
            let mut iter = registers.into_iter();
            Source::X4([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ])
        }
    })
}

impl PtxParser for Stmatrix {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "stmatrix" {
            return Err(unexpected_value(span, &["stmatrix"], opcode));
        }

        let (sync, sync_span) = stream.expect_directive()?;
        if sync != "sync" {
            return Err(unexpected_value(sync_span, &[".sync"], format!(".{sync}")));
        }

        let (aligned, aligned_span) = stream.expect_directive()?;
        if aligned != "aligned" {
            return Err(unexpected_value(
                aligned_span,
                &[".aligned"],
                format!(".{aligned}"),
            ));
        }

        let shape = Shape::parse(stream)?;
        let num = Num::parse(stream)?;

        let trans = consume_directive_if(stream, "trans");

        let state_space = if stream.check(|token| directive_matches(token, "shared")) {
            Some(StateSpace::parse(stream)?)
        } else {
            None
        };

        let data_type = DataType::parse(stream)?;
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Comma)?;
        let source = parse_source(stream, num)?;
        stream.expect(&PtxToken::Semicolon)?;

        Ok(Stmatrix {
            shape,
            num,
            trans,
            state_space,
            data_type,
            address,
            source,
        })
    }
}
