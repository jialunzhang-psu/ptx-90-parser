//! Original PTX specification:
//!
//! st.bulk{.weak}{.shared::cta}  [a], size, initval; // initval must be zero

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
    use crate::r#type::instruction::st_bulk::section_0::*;

    impl PtxParser for StBulkWeakSharedCta {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".bulk"),
                    map(optional(string_p(".weak")), |value, _| value.is_some()),
                    map(optional(string_p(".shared::cta")), |value, _| value
                        .is_some()),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, bulk, weak, shared_cta, a, _, size, _, initval, _), span| {
                    ok!(StBulkWeakSharedCta {
                        bulk = bulk,
                        weak = weak,
                        shared_cta = shared_cta,
                        a = a,
                        size = size,
                        initval = initval,

                    })
                },
            )
        }
    }
}
