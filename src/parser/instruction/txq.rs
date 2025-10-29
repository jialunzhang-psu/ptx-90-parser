use crate::{
    lexer::PtxToken,
    parser::*,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::txq::{Operand, SamplerQuery, TextureLevelQuery, TextureQuery, Txq},
    },
};

fn texture_query_expected() -> [&'static str; 9] {
    [
        ".width",
        ".height",
        ".depth",
        ".channel_data_type",
        ".channel_order",
        ".normalized_coords",
        ".array_size",
        ".num_mipmap_levels",
        ".num_samples",
    ]
}

fn texture_level_query_expected() -> [&'static str; 3] {
    [".width", ".height", ".depth"]
}

fn sampler_query_expected() -> [&'static str; 5] {
    [
        ".force_unnormalized_coords",
        ".filter_mode",
        ".addr_mode_0",
        ".addr_mode_1",
        ".addr_mode_2",
    ]
}

fn first_modifier_expected() -> [&'static str; 15] {
    [
        ".level",
        ".width",
        ".height",
        ".depth",
        ".channel_data_type",
        ".channel_order",
        ".normalized_coords",
        ".array_size",
        ".num_mipmap_levels",
        ".num_samples",
        ".force_unnormalized_coords",
        ".filter_mode",
        ".addr_mode_0",
        ".addr_mode_1",
        ".addr_mode_2",
    ]
}

fn parse_b32(stream: &mut PtxTokenStream) -> Result<(), PtxParseError> {
    let (data_type, span) = stream.expect_directive()?;
    if data_type == "b32" {
        Ok(())
    } else {
        Err(unexpected_value(span, &[".b32"], format!(".{data_type}")))
    }
}

fn parse_address_operand<F>(
    stream: &mut PtxTokenStream,
    mapper: F,
) -> Result<Operand, PtxParseError>
where
    F: FnOnce(VariableSymbol) -> Operand,
{
    stream.expect(&PtxToken::LBracket)?;

    let operand = if stream.check(|token| matches!(token, PtxToken::Identifier(_))) {
        let symbol = VariableSymbol::parse(stream)?;
        mapper(symbol)
    } else if stream.check(|token| matches!(token, PtxToken::Register(_) | PtxToken::LBrace)) {
        Operand::Register(RegisterOperand::parse(stream)?)
    } else {
        let (token, span) = stream.peek()?;
        return Err(unexpected_value(
            span.clone(),
            &["identifier", "register"],
            format!("{token:?}"),
        ));
    };

    stream.expect(&PtxToken::RBracket)?;
    Ok(operand)
}

impl TextureQuery {
    fn from_directive(value: &str, span: Span) -> Result<Self, PtxParseError> {
        match value {
            "width" => Ok(TextureQuery::Width),
            "height" => Ok(TextureQuery::Height),
            "depth" => Ok(TextureQuery::Depth),
            "channel_data_type" => Ok(TextureQuery::ChannelDataType),
            "channel_order" => Ok(TextureQuery::ChannelOrder),
            "normalized_coords" => Ok(TextureQuery::NormalizedCoords),
            "array_size" => Ok(TextureQuery::ArraySize),
            "num_mipmap_levels" => Ok(TextureQuery::NumMipmapLevels),
            "num_samples" => Ok(TextureQuery::NumSamples),
            other => Err(unexpected_value(
                span,
                &texture_query_expected(),
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for TextureQuery {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        TextureQuery::from_directive(&modifier, span)
    }
}

impl TextureLevelQuery {
    fn from_directive(value: &str, span: Span) -> Result<Self, PtxParseError> {
        match value {
            "width" => Ok(TextureLevelQuery::Width),
            "height" => Ok(TextureLevelQuery::Height),
            "depth" => Ok(TextureLevelQuery::Depth),
            other => Err(unexpected_value(
                span,
                &texture_level_query_expected(),
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for TextureLevelQuery {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        TextureLevelQuery::from_directive(&modifier, span)
    }
}

impl SamplerQuery {
    fn from_directive(value: &str, span: Span) -> Result<Self, PtxParseError> {
        match value {
            "force_unnormalized_coords" => Ok(SamplerQuery::ForceUnnormalizedCoords),
            "filter_mode" => Ok(SamplerQuery::FilterMode),
            "addr_mode_0" => Ok(SamplerQuery::AddrMode0),
            "addr_mode_1" => Ok(SamplerQuery::AddrMode1),
            "addr_mode_2" => Ok(SamplerQuery::AddrMode2),
            other => Err(unexpected_value(
                span,
                &sampler_query_expected(),
                format!(".{other}"),
            )),
        }
    }
}

impl PtxParser for SamplerQuery {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let (modifier, span) = stream.expect_directive()?;
        SamplerQuery::from_directive(&modifier, span)
    }
}

impl PtxParser for Txq {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        expect_identifier_value(stream, "txq")?;

        let (modifier, modifier_span) = stream.expect_directive()?;
        match modifier.as_str() {
            "level" => {
                let query = TextureLevelQuery::parse(stream)?;
                parse_b32(stream)?;

                let destination = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Comma)?;

                let address = parse_address_operand(stream, Operand::Texture)?;
                stream.expect(&PtxToken::Comma)?;

                let lod = RegisterOperand::parse(stream)?;
                stream.expect(&PtxToken::Semicolon)?;

                Ok(Txq::TextureLevel {
                    query,
                    destination,
                    address,
                    lod,
                })
            }
            _ => {
                if let Ok(query) =
                    TextureQuery::from_directive(modifier.as_str(), modifier_span.clone())
                {
                    parse_b32(stream)?;

                    let destination = RegisterOperand::parse(stream)?;
                    stream.expect(&PtxToken::Comma)?;

                    let address = parse_address_operand(stream, Operand::Texture)?;
                    stream.expect(&PtxToken::Semicolon)?;

                    Ok(Txq::Texture {
                        query,
                        destination,
                        address,
                    })
                } else if let Ok(query) =
                    SamplerQuery::from_directive(modifier.as_str(), modifier_span.clone())
                {
                    parse_b32(stream)?;

                    let destination = RegisterOperand::parse(stream)?;
                    stream.expect(&PtxToken::Comma)?;

                    let address = parse_address_operand(stream, Operand::Sampler)?;
                    stream.expect(&PtxToken::Semicolon)?;

                    Ok(Txq::Sampler {
                        query,
                        destination,
                        address,
                    })
                } else {
                    Err(unexpected_value(
                        modifier_span,
                        &first_modifier_expected(),
                        format!(".{modifier}"),
                    ))
                }
            }
        }
    }
}
