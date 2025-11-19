//! Original PTX specification:
//!
//! cp.async.mbarrier.arrive{.noinc}{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

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
    use crate::r#type::instruction::cp_async_mbarrier_arrive::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for State {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| State::SharedCta),
                map(string_p(".shared"), |_, _span| State::Shared)
            )
        }
    }

    impl PtxParser for CpAsyncMbarrierArriveNoincStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".mbarrier"),
                    string_p(".arrive"),
                    map(optional(string_p(".noinc")), |value, _| value.is_some()),
                    optional(State::parse()),
                    string_p(".b64"),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, async_, mbarrier, arrive, noinc, state, b64, addr, _), span| {
                    ok!(CpAsyncMbarrierArriveNoincStateB64 {
                        async_ = async_,
                        mbarrier = mbarrier,
                        arrive = arrive,
                        noinc = noinc,
                        state = state,
                        b64 = b64,
                        addr = addr,

                    })
                },
            )
        }
    }
}
