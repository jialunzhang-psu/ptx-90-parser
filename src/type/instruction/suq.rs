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
        ChannelDataType, // .channel_data_type
        ChannelOrder,    // .channel_order
        MemoryLayout,    // .memory_layout
        ArraySize,       // .array_size
        Height,          // .height
        Width,           // .width
        Depth,           // .depth
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SuqQueryB32 {
        pub query: Query,      // .query
        pub b32: (),           // .b32
        pub d: GeneralOperand, // d
        pub a: AddressOperand, // [a]
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Query as Query0;
pub use section_0::SuqQueryB32;
