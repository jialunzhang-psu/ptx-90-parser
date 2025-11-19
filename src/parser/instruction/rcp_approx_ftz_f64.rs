//! Original PTX specification:
//!
//! rcp.approx.ftz.f64  d, a;

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
    use crate::r#type::instruction::rcp_approx_ftz_f64::section_0::*;

    impl PtxParser for RcpApproxFtzF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("rcp"),
                    string_p(".approx"),
                    string_p(".ftz"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, approx, ftz, f64, d, _, a, _), span| {
                    ok!(RcpApproxFtzF64 {
                        approx = approx,
                        ftz = ftz,
                        f64 = f64,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
