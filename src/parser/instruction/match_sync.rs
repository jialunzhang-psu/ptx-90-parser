//! Original PTX specification:
//!
//! match.any.sync.type  d, a, membermask;
//! match.all.sync.type  d{|p}, a, membermask;
//! .type = { .b32, .b64 };

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
    use crate::r#type::instruction::match_sync::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64)
            )
        }
    }

    impl PtxParser for MatchAnySyncType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("match"),
                    string_p(".any"),
                    string_p(".sync"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, any, sync, type_, d, _, a, _, membermask, _), span| {
                    ok!(MatchAnySyncType {
                        any = any,
                        sync = sync,
                        type_ = type_,
                        d = d,
                        a = a,
                        membermask = membermask,

                    })
                },
            )
        }
    }

    impl PtxParser for MatchAllSyncType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("match"),
                    string_p(".all"),
                    string_p(".sync"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, all, sync, type_, d, p, _, a, _, membermask, _), span| {
                    ok!(MatchAllSyncType {
                        all = all,
                        sync = sync,
                        type_ = type_,
                        d = d,
                        p = p,
                        a = a,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}
