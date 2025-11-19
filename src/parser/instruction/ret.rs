//! Original PTX specification:
//!
//! ret{.uni};

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
    use crate::r#type::instruction::ret::section_0::*;

    impl PtxParser for RetUni {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ret"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    semicolon_p()
                ),
                |(_, uni, _), span| {
                    ok!(RetUni {
                        uni = uni,

                    })
                },
            )
        }
    }
}
