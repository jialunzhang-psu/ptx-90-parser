//! Original PTX specification:
//!
//! madc.hilo{.cc}.type  d, a, b, c;
//! .type = { .u32, .s32, .u64, .s64 };
//! .hilo = { .hi, .lo };

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
    use crate::r#type::instruction::madc::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Hilo {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".hi"), |_, _span| Hilo::Hi),
                map(string_p(".lo"), |_, _span| Hilo::Lo)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s64"), |_, _span| Type::S64)
            )
        }
    }

    impl PtxParser for MadcHiloCcType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("madc"),
                    Hilo::parse(),
                    map(optional(string_p(".cc")), |value, _| value.is_some()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, hilo, cc, type_, d, _, a, _, b, _, c, _), span| {
                    ok!(MadcHiloCcType {
                        hilo = hilo,
                        cc = cc,
                        type_ = type_,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}
