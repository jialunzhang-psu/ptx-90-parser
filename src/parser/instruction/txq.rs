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
    use crate::r#type::instruction::txq::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Squery {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".force_unnormalized_coords"), |_, _span| {
                    Squery::ForceUnnormalizedCoords
                }),
                map(string_p(".filter_mode"), |_, _span| Squery::FilterMode),
                map(string_p(".addr_mode_0"), |_, _span| Squery::AddrMode0),
                map(string_p("addr_mode_1"), |_, _span| Squery::AddrMode1),
                map(string_p("addr_mode_2"), |_, _span| Squery::AddrMode2)
            )
        }
    }

    impl PtxParser for Tlquery {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".height"), |_, _span| Tlquery::Height),
                map(string_p(".width"), |_, _span| Tlquery::Width),
                map(string_p(".depth"), |_, _span| Tlquery::Depth)
            )
        }
    }

    impl PtxParser for Tquery {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".channel_data_type"), |_, _span| {
                    Tquery::ChannelDataType
                }),
                map(string_p(".normalized_coords"), |_, _span| {
                    Tquery::NormalizedCoords
                }),
                map(string_p(".num_mipmap_levels"), |_, _span| {
                    Tquery::NumMipmapLevels
                }),
                map(string_p(".channel_order"), |_, _span| Tquery::ChannelOrder),
                map(string_p(".num_samples"), |_, _span| Tquery::NumSamples),
                map(string_p(".array_size"), |_, _span| Tquery::ArraySize),
                map(string_p(".height"), |_, _span| Tquery::Height),
                map(string_p(".width"), |_, _span| Tquery::Width),
                map(string_p(".depth"), |_, _span| Tquery::Depth)
            )
        }
    }

    impl PtxParser for TxqTqueryB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("txq"),
                    Tquery::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, tquery, b32, d, _, a, _), span| {
                    ok!(TxqTqueryB32 {
                        tquery = tquery,
                        b32 = b32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for TxqLevelTlqueryB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("txq"),
                    string_p(".level"),
                    Tlquery::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, level, tlquery, b32, d, _, a, _, lod, _), span| {
                    ok!(TxqLevelTlqueryB32 {
                        level = level,
                        tlquery = tlquery,
                        b32 = b32,
                        d = d,
                        a = a,
                        lod = lod,

                    })
                },
            )
        }
    }

    impl PtxParser for TxqSqueryB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("txq"),
                    Squery::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, squery, b32, d, _, a, _), span| {
                    ok!(TxqSqueryB32 {
                        squery = squery,
                        b32 = b32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
