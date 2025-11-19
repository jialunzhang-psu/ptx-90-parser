//! Original PTX specification:
//!
//! vote.mode.pred  d, {!}a;
//! vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask
//! .mode = { .all, .any, .uni };

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
    use crate::r#type::instruction::vote::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".all"), |_, _span| Mode::All),
                map(string_p(".any"), |_, _span| Mode::Any),
                map(string_p(".uni"), |_, _span| Mode::Uni)
            )
        }
    }

    impl PtxParser for VoteModePred {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vote"),
                    Mode::parse(),
                    string_p(".pred"),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, mode, pred, d, _, a_op, a, _), span| {
                    ok!(VoteModePred {
                        mode = mode,
                        pred = pred,
                        d = d,
                        a_op = a_op,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for VoteBallotB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vote"),
                    string_p(".ballot"),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ballot, b32, d, _, a_op, a, _), span| {
                    ok!(VoteBallotB32 {
                        ballot = ballot,
                        b32 = b32,
                        d = d,
                        a_op = a_op,
                        a = a,

                    })
                },
            )
        }
    }
}
