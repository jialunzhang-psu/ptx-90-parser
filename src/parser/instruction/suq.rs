//! Original PTX specification:
//!
//! suq.query.b32   d, [a];
//! .query = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .array_size, .memory_layout };

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::suq::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Query {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".channel_data_type"), |_, _span| {
                    Query::ChannelDataType
                }),
                map(string_p(".channel_order"), |_, _span| Query::ChannelOrder),
                map(string_p(".memory_layout"), |_, _span| Query::MemoryLayout),
                map(string_p(".array_size"), |_, _span| Query::ArraySize),
                map(string_p(".height"), |_, _span| Query::Height),
                map(string_p(".width"), |_, _span| Query::Width),
                map(string_p(".depth"), |_, _span| Query::Depth)
            )
        }
    }

    impl PtxParser for SuqQueryB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("suq"),
                    Query::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, query, b32, d, _, a, _), span| {
                    ok!(SuqQueryB32 {
                        query = query,
                        b32 = b32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
