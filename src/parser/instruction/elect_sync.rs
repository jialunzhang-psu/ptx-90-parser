//! Original PTX specification:
//!
//! elect.sync d|p, membermask;

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
    use crate::r#type::instruction::elect_sync::section_0::*;

    impl PtxParser for ElectSync {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("elect"),
                    string_p(".sync"),
                    GeneralOperand::parse(),
                    pipe_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, d, _, p, _, membermask, _), span| {
                    ok!(ElectSync {
                        sync = sync,
                        d = d,
                        p = p,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}
