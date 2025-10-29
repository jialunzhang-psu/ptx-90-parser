use crate::r#type::common::{RegisterOperand, VariableSymbol};

/// `txq.tquery.b32 d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Txq {
    /// `txq.tquery.b32 d, [a];`
    Texture {
        /// `.tquery`
        query: TextureQuery,
        /// `d`
        destination: RegisterOperand,
        /// `[a]`
        address: Operand,
    },
    /// `txq.level.tlquery.b32 d, [a], lod;`
    TextureLevel {
        /// `.tlquery`
        query: TextureLevelQuery,
        /// `d`
        destination: RegisterOperand,
        /// `[a]`
        address: Operand,
        /// `lod`
        lod: RegisterOperand,
    },
    /// `txq.squery.b32 d, [a];`
    Sampler {
        /// `.squery`
        query: SamplerQuery,
        /// `d`
        destination: RegisterOperand,
        /// `[a]`
        address: Operand,
    },
}

/// `.tquery = { .width, .height, .depth, .channel_data_type, .channel_order, .normalized_coords, .array_size, .num_mipmap_levels, .num_samples };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureQuery {
    /// `.width`
    Width,
    /// `.height`
    Height,
    /// `.depth`
    Depth,
    /// `.channel_data_type`
    ChannelDataType,
    /// `.channel_order`
    ChannelOrder,
    /// `.normalized_coords`
    NormalizedCoords,
    /// `.array_size`
    ArraySize,
    /// `.num_mipmap_levels`
    NumMipmapLevels,
    /// `.num_samples`
    NumSamples,
}

/// `.tlquery = { .width, .height, .depth };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureLevelQuery {
    /// `.width`
    Width,
    /// `.height`
    Height,
    /// `.depth`
    Depth,
}

/// `.squery = { .force_unnormalized_coords, .filter_mode, .addr_mode_0, .addr_mode_1, .addr_mode_2 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplerQuery {
    /// `.force_unnormalized_coords`
    ForceUnnormalizedCoords,
    /// `.filter_mode`
    FilterMode,
    /// `.addr_mode_0`
    AddrMode0,
    /// `.addr_mode_1`
    AddrMode1,
    /// `.addr_mode_2`
    AddrMode2,
}

/// `a = { .texref, .samplerref, .u64 }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// `.texref`
    Texture(VariableSymbol),
    /// `.samplerref`
    Sampler(VariableSymbol),
    /// `.u64`
    Register(RegisterOperand),
}
