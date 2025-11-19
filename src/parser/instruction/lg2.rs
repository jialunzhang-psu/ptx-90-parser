//! Original PTX specification:
//!
//! lg2.approx{.ftz}.f32  d, a;

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
    use crate::r#type::instruction::lg2::section_0::*;

    impl PtxParser for Lg2ApproxFtzF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("lg2"),
                    string_p(".approx"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f32, d, _, a, _), span| {
                    ok!(Lg2ApproxFtzF32 {
                        approx = approx,
                        ftz = ftz,
                        f32 = f32,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
