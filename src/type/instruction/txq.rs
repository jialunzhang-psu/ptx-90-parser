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
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tquery {
        Width, // .width
        Height, // .height
        Depth, // .depth
        ChannelDataType, // .channel_data_type
        ChannelOrder, // .channel_order
        NormalizedCoords, // .normalized_coords
        ArraySize, // .array_size
        NumMipmapLevels, // .num_mipmap_levels
        NumSamples, // .num_samples
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tlquery {
        Width, // .width
        Height, // .height
        Depth, // .depth
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Squery {
        ForceUnnormalizedCoords, // .force_unnormalized_coords
        FilterMode, // .filter_mode
        AddrMode0, // .addr_mode_0
        AddrMode1, // addr_mode_1
        AddrMode2, // addr_mode_2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TxqTqueryB32 {
        pub tquery: Tquery, // .tquery
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TxqLevelTlqueryB32 {
        pub level: (), // .level
        pub tlquery: Tlquery, // .tlquery
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
        pub lod: Operand, // lod
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TxqSqueryB32 {
        pub squery: Squery, // .squery
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

}
