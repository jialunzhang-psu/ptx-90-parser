//! Original PTX specification:
//!
//! mbarrier.pending_count.b64 count, state;

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
    use crate::r#type::instruction::mbarrier_pending_count::section_0::*;

    impl PtxParser for MbarrierPendingCountB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".pending_count"),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, pending_count, b64, count, _, state, _), span| {
                    ok!(MbarrierPendingCountB64 {
                        pending_count = pending_count,
                        b64 = b64,
                        count = count,
                        state = state,

                    })
                },
            )
        }
    }
}
