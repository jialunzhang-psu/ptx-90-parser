//! Original PTX specification:
//!
//! pmevent a;         // trigger a single performance monitor event
//! pmevent.mask a;    // trigger one or more performance monitor events

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
    use crate::r#type::instruction::pmevent::section_0::*;

    impl PtxParser for Pmevent {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(string_p("pmevent"), GeneralOperand::parse(), semicolon_p()),
                |(_, a, _), span| {
                    ok!(Pmevent {
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for PmeventMask {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("pmevent"),
                    string_p(".mask"),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, mask, a, _), span| {
                    ok!(PmeventMask {
                        mask = mask,
                        a = a,

                    })
                },
            )
        }
    }
}
