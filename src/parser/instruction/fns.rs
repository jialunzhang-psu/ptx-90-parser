//! Original PTX specification:
//!
//! fns.b32 d, mask, base, offset;

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
    use crate::r#type::instruction::fns::section_0::*;

    impl PtxParser for FnsB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fns"),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b32, d, _, mask, _, base, _, offset, _), span| {
                    ok!(FnsB32 {
                        b32 = b32,
                        d = d,
                        mask = mask,
                        base = base,
                        offset = offset,

                    })
                },
            )
        }
    }
}
