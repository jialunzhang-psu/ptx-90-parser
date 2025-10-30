use crate::{
    lexer::PtxToken,
    r#type::instruction::txq::{Operand, SamplerQuery, TextureLevelQuery, TextureQuery, Txq},
    unparser::*,
};

impl PtxUnparser for TextureQuery {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            TextureQuery::Width => "width",
            TextureQuery::Height => "height",
            TextureQuery::Depth => "depth",
            TextureQuery::ChannelDataType => "channel_data_type",
            TextureQuery::ChannelOrder => "channel_order",
            TextureQuery::NormalizedCoords => "normalized_coords",
            TextureQuery::ArraySize => "array_size",
            TextureQuery::NumMipmapLevels => "num_mipmap_levels",
            TextureQuery::NumSamples => "num_samples",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for TextureLevelQuery {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            TextureLevelQuery::Width => "width",
            TextureLevelQuery::Height => "height",
            TextureLevelQuery::Depth => "depth",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for SamplerQuery {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            SamplerQuery::ForceUnnormalizedCoords => "force_unnormalized_coords",
            SamplerQuery::FilterMode => "filter_mode",
            SamplerQuery::AddrMode0 => "addr_mode_0",
            SamplerQuery::AddrMode1 => "addr_mode_1",
            SamplerQuery::AddrMode2 => "addr_mode_2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Operand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBracket);
        match self {
            Operand::Texture(symbol) => symbol.unparse_tokens(tokens),
            Operand::Sampler(symbol) => symbol.unparse_tokens(tokens),
            Operand::Register(register) => register.unparse_tokens(tokens),
        }
        tokens.push(PtxToken::RBracket);
    }
}

impl PtxUnparser for Txq {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "txq");
        match self {
            Txq::Texture {
                query,
                destination,
                address,
            } => {
                query.unparse_tokens(tokens);
                push_directive(tokens, "b32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Txq::TextureLevel {
                query,
                destination,
                address,
                lod,
            } => {
                push_directive(tokens, "level");
                query.unparse_tokens(tokens);
                push_directive(tokens, "b32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                lod.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Txq::Sampler {
                query,
                destination,
                address,
            } => {
                query.unparse_tokens(tokens);
                push_directive(tokens, "b32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
