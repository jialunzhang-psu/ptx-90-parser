//! Original PTX specification:
//!
//! activemask.b32 d;

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
    use crate::r#type::instruction::activemask::section_0::*;

    impl PtxParser for ActivemaskB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("activemask"),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b32, d, _), span| {
                    ok!(ActivemaskB32 {
                        b32 = b32,
                        d = d,

                    })
                },
            )
        }
    }
}
