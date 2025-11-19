//! Original PTX specification:
//!
//! txq.tquery.b32         d, [a];       // texture attributes
//! txq.level.tlquery.b32  d, [a], lod;  // texture attributes
//! txq.squery.b32         d, [a];       // sampler attributes
//! .tquery  = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .normalized_coords, .array_size,
//! .num_mipmap_levels, .num_samples};
//! .tlquery = { .width, .height, .depth };
//! .squery  = { .force_unnormalized_coords, .filter_mode,
//! .addr_mode_0, addr_mode_1, addr_mode_2 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tquery {
        ChannelDataType,  // .channel_data_type
        NormalizedCoords, // .normalized_coords
        NumMipmapLevels,  // .num_mipmap_levels
        ChannelOrder,     // .channel_order
        NumSamples,       // .num_samples
        ArraySize,        // .array_size
        Height,           // .height
        Width,            // .width
        Depth,            // .depth
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tlquery {
        Height, // .height
        Width,  // .width
        Depth,  // .depth
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Squery {
        ForceUnnormalizedCoords, // .force_unnormalized_coords
        FilterMode,              // .filter_mode
        AddrMode0,               // .addr_mode_0
        AddrMode1,               // addr_mode_1
        AddrMode2,               // addr_mode_2
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct TxqTqueryB32 {
        pub tquery: Tquery,    // .tquery
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct TxqLevelTlqueryB32 {
        pub level: (),           // .level
        pub tlquery: Tlquery,    // .tlquery
        pub b32: (),             // .b32
        pub d: GeneralOperand,   // d
        pub a: AddressOperand,   // [a]
        pub lod: GeneralOperand, // lod
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct TxqSqueryB32 {
        pub squery: Squery,    // .squery
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Squery as Squery0;
pub use section_0::Tlquery as Tlquery0;
pub use section_0::Tquery as Tquery0;
pub use section_0::TxqLevelTlqueryB32;
pub use section_0::TxqSqueryB32;
pub use section_0::TxqTqueryB32;
