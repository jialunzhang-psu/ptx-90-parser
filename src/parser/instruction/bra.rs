//! Original PTX specification:
//!
//! bra{.uni}  tgt;           // tgt is a label
//! bra{.uni}  tgt;           // unconditional branch

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
    use crate::r#type::instruction::bra::section_0::*;

    impl PtxParser for BraUni {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bra"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, tgt, _), span| {
                    ok!(BraUni {
                        uni = uni,
                        tgt = tgt,

                    })
                },
            )
        }
    }

    impl PtxParser for BraUni1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bra"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, uni, tgt, _), span| {
                    ok!(BraUni1 {
                        uni = uni,
                        tgt = tgt,

                    })
                },
            )
        }
    }
}
