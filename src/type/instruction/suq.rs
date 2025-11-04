//! Original PTX specification:
//!
//! suq.query.b32   d, [a];
//! .query = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .array_size, .memory_layout };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Query {
        Width, // .width
        Height, // .height
        Depth, // .depth
        ChannelDataType, // .channel_data_type
        ChannelOrder, // .channel_order
        ArraySize, // .array_size
        MemoryLayout, // .memory_layout
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SuqQueryB32 {
        pub query: Query, // .query
        pub b32: (), // .b32
        pub d: Operand, // d
        pub a: AddressOperand, // [a]
    }

}
