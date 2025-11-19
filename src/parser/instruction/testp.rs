//! Original PTX specification:
//!
//! testp.op.type  p, a;  // result is .pred
//! .op   = { .finite, .infinite,
//! .number, .notanumber,
//! .normal, .subnormal };
//! .type = { .f32, .f64 };

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
    use crate::r#type::instruction::testp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".notanumber"), |_, _span| Op::Notanumber),
                map(string_p(".subnormal"), |_, _span| Op::Subnormal),
                map(string_p(".infinite"), |_, _span| Op::Infinite),
                map(string_p(".finite"), |_, _span| Op::Finite),
                map(string_p(".number"), |_, _span| Op::Number),
                map(string_p(".normal"), |_, _span| Op::Normal)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64)
            )
        }
    }

    impl PtxParser for TestpOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("testp"),
                    Op::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, op, type_, p, _, a, _), span| {
                    ok!(TestpOpType {
                        op = op,
                        type_ = type_,
                        p = p,
                        a = a,

                    })
                },
            )
        }
    }
}
