//! Original PTX specification:
//!
//! redux.sync.op.type dst, src, membermask;
//! .op   = {.add, .min, .max};
//! .type = {.u32, .s32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op.b32 dst, src, membermask;
//! .op   = {.and, .or, .xor};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! redux.sync.op{.abs}{.NaN}.f32 dst, src, membermask;
//! .op   = { .min, .max };

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
    use crate::r#type::instruction::redux_sync::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".s32"), |_, _span| Type::S32)
            )
        }
    }

    impl PtxParser for ReduxSyncOpType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("redux"),
                    string_p(".sync"),
                    Op::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, op, type_, dst, _, src, _, membermask, _), span| {
                    ok!(ReduxSyncOpType {
                        sync = sync,
                        op = op,
                        type_ = type_,
                        dst = dst,
                        src = src,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::redux_sync::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for ReduxSyncOpB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("redux"),
                    string_p(".sync"),
                    Op::parse(),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, op, b32, dst, _, src, _, membermask, _), span| {
                    ok!(ReduxSyncOpB32 {
                        sync = sync,
                        op = op,
                        b32 = b32,
                        dst = dst,
                        src = src,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::redux_sync::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max)
            )
        }
    }

    impl PtxParser for ReduxSyncOpAbsNanF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("redux"),
                    string_p(".sync"),
                    Op::parse(),
                    map(optional(string_p(".abs")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, op, abs, nan, f32, dst, _, src, _, membermask, _), span| {
                    ok!(ReduxSyncOpAbsNanF32 {
                        sync = sync,
                        op = op,
                        abs = abs,
                        nan = nan,
                        f32 = f32,
                        dst = dst,
                        src = src,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}
