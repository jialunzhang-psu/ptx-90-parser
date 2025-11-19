//! Original PTX specification:
//!
//! mapa{.space}.type          d, a, b;
//! // Maps shared memory address in register a into CTA b.
//! // mapa.shared::cluster.type  d, a, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh, b;
//! // Maps shared memory variable into CTA b.
//! // mapa.shared::cluster.type  d, sh + imm, b;
//! // Maps generic address in register a into CTA b.
//! // mapa.type                  d, a, b;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

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
    use crate::r#type::instruction::mapa::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Space::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for MapaSpaceType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mapa"),
                    optional(Space::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, space, type_, d, _, a, _, b, _), span| {
                    ok!(MapaSpaceType {
                        space = space,
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
