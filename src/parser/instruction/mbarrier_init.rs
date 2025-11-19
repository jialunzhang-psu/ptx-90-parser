//! Original PTX specification:
//!
//! mbarrier.init{.state}.b64 [addr], count;
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
    use crate::r#type::instruction::mbarrier_init::section_0::*;

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

    impl PtxParser for MbarrierInitStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".init"),
                    optional(State::parse()),
                    string_p(".b64"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, init, state, b64, addr, _, count, _), span| {
                    ok!(MbarrierInitStateB64 {
                        init = init,
                        state = state,
                        b64 = b64,
                        addr = addr,
                        count = count,

                    })
                },
            )
        }
    }
}
