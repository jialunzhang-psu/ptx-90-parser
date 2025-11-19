//! Original PTX specification:
//!
//! ldu{.ss}.type      d, [a];       // load from address
//! ldu{.ss}.vec.type  d, [a];       // vec load from address
//! .ss   = { .global };             // state space
//! .vec  = { .v2, .v4 };
//! .type = { .b8, .b16, .b32, .b64, .b128,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
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
    use crate::r#type::instruction::ldu::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Ss::Global))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b128"), |_, _span| Type::B128),
                map(string_p(".b16"), |_, _span| Type::B16),
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u16"), |_, _span| Type::U16),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64),
                map(string_p(".b8"), |_, _span| Type::B8),
                map(string_p(".u8"), |_, _span| Type::U8),
                map(string_p(".s8"), |_, _span| Type::S8)
            )
        }
    }

    impl PtxParser for Vec {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".v2"), |_, _span| Vec::V2),
                map(string_p(".v4"), |_, _span| Vec::V4)
            )
        }
    }

    impl PtxParser for LduSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ldu"),
                    optional(Ss::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ss, type_, d, _, a, _), span| {
                    ok!(LduSsType {
                        ss = ss,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for LduSsVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ldu"),
                    optional(Ss::parse()),
                    Vec::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ss, vec, type_, d, _, a, _), span| {
                    ok!(LduSsVecType {
                        ss = ss,
                        vec = vec,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
