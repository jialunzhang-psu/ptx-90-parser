//! Original PTX specification:
//!
//! sured.b.op.geom.ctype.mode [a,b],c; // byte addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .u32, .u64, .s32, .b32, .s64 };  // for sured.b
//! .mode  = { .trap, .clamp, .zero };
//! ----------------------------------------------------
//! sured.p.op.geom.ctype.mode [a,b],c; // sample addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .b32, .b64 };                    // for sured.p
//! .mode  = { .trap, .clamp, .zero };

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
    use crate::r#type::instruction::sured::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Ctype::U32),
                map(string_p(".u64"), |_, _span| Ctype::U64),
                map(string_p(".s32"), |_, _span| Ctype::S32),
                map(string_p(".b32"), |_, _span| Ctype::B32),
                map(string_p(".s64"), |_, _span| Ctype::S64)
            )
        }
    }

    impl PtxParser for Geom {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Geom::_1d),
                map(string_p(".2d"), |_, _span| Geom::_2d),
                map(string_p(".3d"), |_, _span| Geom::_3d)
            )
        }
    }

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".clamp"), |_, _span| Mode::Clamp),
                map(string_p(".trap"), |_, _span| Mode::Trap),
                map(string_p(".zero"), |_, _span| Mode::Zero)
            )
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max),
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for SuredBOpGeomCtypeMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sured"),
                    string_p(".b"),
                    Op::parse(),
                    Geom::parse(),
                    Ctype::parse(),
                    Mode::parse(),
                    TexHandler2::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b, op, geom, ctype, mode, a, _, c, _), span| {
                    ok!(SuredBOpGeomCtypeMode {
                        b = b,
                        op = op,
                        geom = geom,
                        ctype = ctype,
                        mode = mode,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::sured::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Ctype::B32),
                map(string_p(".b64"), |_, _span| Ctype::B64)
            )
        }
    }

    impl PtxParser for Geom {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Geom::_1d),
                map(string_p(".2d"), |_, _span| Geom::_2d),
                map(string_p(".3d"), |_, _span| Geom::_3d)
            )
        }
    }

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".clamp"), |_, _span| Mode::Clamp),
                map(string_p(".trap"), |_, _span| Mode::Trap),
                map(string_p(".zero"), |_, _span| Mode::Zero)
            )
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op::Add),
                map(string_p(".min"), |_, _span| Op::Min),
                map(string_p(".max"), |_, _span| Op::Max),
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for SuredPOpGeomCtypeMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sured"),
                    string_p(".p"),
                    Op::parse(),
                    Geom::parse(),
                    Ctype::parse(),
                    Mode::parse(),
                    TexHandler2::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, p, op, geom, ctype, mode, a, _, c, _), span| {
                    ok!(SuredPOpGeomCtypeMode {
                        p = p,
                        op = op,
                        geom = geom,
                        ctype = ctype,
                        mode = mode,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }
}
