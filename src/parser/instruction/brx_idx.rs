//! Original PTX specification:
//!
//! brx.idx{.uni} index, tlist;
//! brx.idx{.uni} index, tlist;

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
    use crate::r#type::instruction::brx_idx::section_0::*;

    impl PtxParser for BrxIdxUni {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("brx"),
                    string_p(".idx"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, idx, uni, index, _, tlist, _), span| {
                    ok!(BrxIdxUni {
                        idx = idx,
                        uni = uni,
                        index = index,
                        tlist = tlist,

                    })
                },
            )
        }
    }

    impl PtxParser for BrxIdxUni1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("brx"),
                    string_p(".idx"),
                    map(optional(string_p(".uni")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, idx, uni, index, _, tlist, _), span| {
                    ok!(BrxIdxUni1 {
                        idx = idx,
                        uni = uni,
                        index = index,
                        tlist = tlist,

                    })
                },
            )
        }
    }
}
