use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{common::*, instruction::createpolicy::*},
};

fn parse_primary_and_secondary(
    stream: &mut PtxTokenStream,
) -> Result<(PrimaryPriority, Option<SecondaryPriority>), PtxParseError> {
    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if name == "level"
        )
    }) {
        let span = stream.peek().map(|(_, span)| span.clone()).unwrap_or(0..0);
        return Err(unexpected_value(span, &[".L2"], ".level".to_string()));
    }
    let primary_priority = PrimaryPriority::parse(stream)?;

    if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if name == "level"
        )
    }) {
        let span = stream.peek().map(|(_, span)| span.clone()).unwrap_or(0..0);
        return Err(unexpected_value(span, &[".L2"], ".level".to_string()));
    }

    let secondary_priority = if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if name == "L2"
        )
    }) {
        Some(SecondaryPriority::parse(stream)?)
    } else {
        None
    };

    Ok((primary_priority, secondary_priority))
}

fn expect_b64(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    expect_directive_value(stream, "b64")
}

impl PtxParser for PrimaryPriority {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (level, level_span) = stream.expect_directive()?;
        if level.as_str() != "L2" {
            return Err(unexpected_value(level_span, &[".L2"], format!(".{level}")));
        }

        stream.expect_double_colon()?;
        let (name, name_span) = stream.expect_identifier()?;
        match name.as_str() {
            "evict_last" => Ok(PrimaryPriority::L2EvictLast),
            "evict_normal" => Ok(PrimaryPriority::L2EvictNormal),
            "evict_first" => Ok(PrimaryPriority::L2EvictFirst),
            "evict_unchanged" => Ok(PrimaryPriority::L2EvictUnchanged),
            other => Err(unexpected_value(
                name_span,
                &[
                    "evict_last",
                    "evict_normal",
                    "evict_first",
                    "evict_unchanged",
                ],
                other,
            )),
        }
    }
}

impl PtxParser for SecondaryPriority {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (level, level_span) = stream.expect_directive()?;
        if level.as_str() != "L2" {
            return Err(unexpected_value(level_span, &[".L2"], format!(".{level}")));
        }

        stream.expect_double_colon()?;
        let (name, name_span) = stream.expect_identifier()?;
        match name.as_str() {
            "evict_first" => Ok(SecondaryPriority::L2EvictFirst),
            "evict_unchanged" => Ok(SecondaryPriority::L2EvictUnchanged),
            other => Err(unexpected_value(
                name_span,
                &["evict_first", "evict_unchanged"],
                other,
            )),
        }
    }
}

impl PtxParser for Level {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        match directive.as_str() {
            "L2" => Ok(Level::L2),
            other => Err(unexpected_value(span, &[".L2"], format!(".{other}"))),
        }
    }
}

fn parse_range(stream: &mut PtxTokenStream) -> Result<Createpolicy, PtxParseError> {
    let global = consume_directive_if(stream, "global");
    let (primary_priority, secondary_priority) = parse_primary_and_secondary(stream)?;
    expect_b64(stream)?;

    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let primary_size = Operand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let total_size = Operand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Createpolicy::Range {
        global,
        primary_priority,
        secondary_priority,
        destination,
        address,
        primary_size,
        total_size,
    })
}

fn parse_fractional(stream: &mut PtxTokenStream) -> Result<Createpolicy, PtxParseError> {
    let (primary_priority, secondary_priority) = parse_primary_and_secondary(stream)?;
    expect_b64(stream)?;

    let destination = RegisterOperand::parse(stream)?;
    let fraction = if stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        Some(Immediate::parse(stream)?)
    } else {
        None
    };
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Createpolicy::Fractional {
        primary_priority,
        secondary_priority,
        destination,
        fraction,
    })
}

fn parse_convert(stream: &mut PtxTokenStream) -> Result<Createpolicy, PtxParseError> {
    let level = Level::parse(stream)?;
    expect_b64(stream)?;

    let destination = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Comma)?;
    let access_property = RegisterOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Createpolicy::Convert {
        level,
        destination,
        access_property,
    })
}

impl PtxParser for Createpolicy {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        if opcode != "createpolicy" {
            return Err(unexpected_value(span, &["createpolicy"], opcode));
        }

        let (modifier, modifier_span) = stream.expect_directive()?;
        match modifier.as_str() {
            "range" => parse_range(stream),
            "fractional" => parse_fractional(stream),
            "cvt" => parse_convert(stream),
            other => Err(unexpected_value(
                modifier_span,
                &[".range", ".fractional", ".cvt"],
                format!(".{other}"),
            )),
        }
    }
}
