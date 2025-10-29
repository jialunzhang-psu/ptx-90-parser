use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span, unexpected_value},
    r#type::{common::AddressOperand, instruction::prefetch::*},
};

fn space_from_directive(directive: &str, span: Span) -> Result<Space, PtxParseError> {
    match directive {
        "global" => Ok(Space::Global),
        "local" => Ok(Space::Local),
        other => Err(unexpected_value(
            span,
            &[".global", ".local"],
            format!(".{other}"),
        )),
    }
}

fn space_to_directive(space: Space) -> &'static str {
    match space {
        Space::Global => ".global",
        Space::Local => ".local",
    }
}

fn level_from_directive(directive: &str, span: Span) -> Result<Level, PtxParseError> {
    match directive {
        "L1" => Ok(Level::L1),
        "L2" => Ok(Level::L2),
        other => Err(unexpected_value(span, &[".L1", ".L2"], format!(".{other}"))),
    }
}

fn tensor_map_space_from_directive(
    directive: &str,
    span: Span,
) -> Result<TensorMapSpace, PtxParseError> {
    match directive {
        "const" => Ok(TensorMapSpace::Const),
        "param" => Ok(TensorMapSpace::Param),
        other => Err(unexpected_value(
            span,
            &[".const", ".param"],
            format!(".{other}"),
        )),
    }
}

impl PtxParser for Space {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        space_from_directive(&directive, span)
    }
}

impl PtxParser for Level {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        level_from_directive(&directive, span)
    }
}

impl PtxParser for Eviction {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (name, span) = stream.expect_identifier()?;
        match name.as_str() {
            "evict_last" => Ok(Eviction::L2EvictLast),
            "evict_normal" => Ok(Eviction::L2EvictNormal),
            other => Err(unexpected_value(
                span,
                &["evict_last", "evict_normal"],
                other,
            )),
        }
    }
}

impl PtxParser for TensorMapSpace {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (directive, span) = stream.expect_directive()?;
        tensor_map_space_from_directive(&directive, span)
    }
}

fn parse_prefetch_tensormap(stream: &mut PtxTokenStream) -> Result<Prefetch, PtxParseError> {
    let space = if stream.check(|token| {
        matches!(
            token,
            PtxToken::Directive(name) if name == "const" || name == "param"
        )
    }) {
        Some(TensorMapSpace::parse(stream)?)
    } else {
        None
    };

    let (directive, span) = stream.expect_directive()?;
    if directive != "tensormap" {
        return Err(unexpected_value(
            span,
            &[".tensormap"],
            format!(".{directive}"),
        ));
    }

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;

    Ok(Prefetch::TensorMap { space, address })
}

fn parse_prefetch_datacache(stream: &mut PtxTokenStream) -> Result<Prefetch, PtxParseError> {
    let (level_directive, level_span) = stream.expect_directive()?;
    let level = level_from_directive(&level_directive, level_span.clone())?;

    if stream.check(|token| matches!(token, PtxToken::Colon)) {
        let (_, colon_span) = stream.peek()?;
        return Err(unexpected_value(
            colon_span.clone(),
            &["["],
            format!("{:?}", PtxToken::Colon),
        ));
    }

    let address = AddressOperand::parse(stream)?;
    stream.expect(&PtxToken::Semicolon)?;
    Ok(Prefetch::DataCache {
        space: None,
        level,
        address,
    })
}

fn parse_prefetch_with_space(
    stream: &mut PtxTokenStream,
    space: Space,
    space_span: Span,
) -> Result<Prefetch, PtxParseError> {
    let (level_directive, level_span) = stream.expect_directive()?;
    let level = level_from_directive(&level_directive, level_span.clone())?;

    if stream.check(|token| matches!(token, PtxToken::Colon)) {
        stream.expect_double_colon()?;
        if !matches!(space, Space::Global) {
            return Err(unexpected_value(
                space_span,
                &[".global"],
                space_to_directive(space).to_string(),
            ));
        }
        let eviction = Eviction::parse(stream)?;
        if level != Level::L2 {
            return Err(unexpected_value(
                level_span,
                &[".L2"],
                format!(".{level_directive}"),
            ));
        }
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Prefetch::GlobalEviction { eviction, address })
    } else {
        let address = AddressOperand::parse(stream)?;
        stream.expect(&PtxToken::Semicolon)?;
        Ok(Prefetch::DataCache {
            space: Some(space),
            level,
            address,
        })
    }
}

impl PtxParser for Prefetch {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (opcode, span) = stream.expect_identifier()?;
        match opcode.as_str() {
            "prefetch" => {
                if stream.check(|token| {
                    matches!(
                        token,
                        PtxToken::Directive(name)
                            if name == "const" || name == "param" || name == "tensormap"
                    )
                }) {
                    return parse_prefetch_tensormap(stream);
                }

                if stream.check(|token| {
                    matches!(
                        token,
                        PtxToken::Directive(name) if name == "global" || name == "local"
                    )
                }) {
                    let (directive, space_span) = stream.expect_directive()?;
                    let space = space_from_directive(&directive, space_span.clone())?;
                    return parse_prefetch_with_space(stream, space, space_span);
                }

                parse_prefetch_datacache(stream)
            }
            "prefetchu" => {
                let (level_directive, level_span) = stream.expect_directive()?;
                let level = level_from_directive(&level_directive, level_span.clone())?;
                if level != Level::L1 {
                    return Err(unexpected_value(
                        level_span,
                        &[".L1"],
                        format!(".{level_directive}"),
                    ));
                }

                let address = AddressOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;
                Ok(Prefetch::Uniform { address })
            }
            other => Err(unexpected_value(span, &["prefetch", "prefetchu"], other)),
        }
    }
}
