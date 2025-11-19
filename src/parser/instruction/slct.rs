//! Original PTX specification:
//!
//! slct.dtype.s32        d, a, b, c;
//! slct{.ftz}.dtype.f32  d, a, b, c;
//! .dtype = { .b16, .b32, .b64,
//! .u16, .u32, .u64,
//! .s16, .s32, .s64,
//! .f32, .f64 };

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
    use crate::r#type::instruction::slct::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Dtype::B16),
                map(string_p(".b32"), |_, _span| Dtype::B32),
                map(string_p(".b64"), |_, _span| Dtype::B64),
                map(string_p(".u16"), |_, _span| Dtype::U16),
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".u64"), |_, _span| Dtype::U64),
                map(string_p(".s16"), |_, _span| Dtype::S16),
                map(string_p(".s32"), |_, _span| Dtype::S32),
                map(string_p(".s64"), |_, _span| Dtype::S64),
                map(string_p(".f32"), |_, _span| Dtype::F32),
                map(string_p(".f64"), |_, _span| Dtype::F64)
            )
        }
    }

    impl PtxParser for SlctDtypeS32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("slct"),
                    Dtype::parse(),
                    string_p(".s32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, s32, d, _, a, _, b, _, c, _), span| {
                    ok!(SlctDtypeS32 {
                        dtype = dtype,
                        s32 = s32,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SlctFtzDtypeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("slct"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    Dtype::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, dtype, f32, d, _, a, _, b, _, c, _), span| {
                    ok!(SlctFtzDtypeF32 {
                        ftz = ftz,
                        dtype = dtype,
                        f32 = f32,
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
