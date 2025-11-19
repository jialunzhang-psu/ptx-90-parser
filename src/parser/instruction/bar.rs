//! Original PTX specification:
//!
//! barrier{.cta}.sync{.aligned}      a{, b};
//! barrier{.cta}.arrive{.aligned}    a, b;
//! barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
//! barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
//! bar{.cta}.sync      a{, b};
//! bar{.cta}.arrive    a, b;
//! bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
//! bar{.cta}.red.op.pred   p, a{, b}, {!}c;
//! .op = { .and, .or };

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
    use crate::r#type::instruction::bar::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".and"), |_, _span| Op::And),
                map(string_p(".or"), |_, _span| Op::Or)
            )
        }
    }

    impl PtxParser for BarrierCtaSyncAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".sync"),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, cta, sync, aligned, a, b, _), span| {
                    ok!(BarrierCtaSyncAligned {
                        cta = cta,
                        sync = sync,
                        aligned = aligned,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for BarrierCtaArriveAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".arrive"),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, arrive, aligned, a, _, b, _), span| {
                    ok!(BarrierCtaArriveAligned {
                        cta = cta,
                        arrive = arrive,
                        aligned = aligned,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for BarrierCtaRedPopcAlignedU32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".red"),
                    string_p(".popc"),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    string_p(".u32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, red, popc, aligned, u32, d, _, a, b, _, c_op, c, _), span| {
                    ok!(BarrierCtaRedPopcAlignedU32 {
                        cta = cta,
                        red = red,
                        popc = popc,
                        aligned = aligned,
                        u32 = u32,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for BarrierCtaRedOpAlignedPred {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".red"),
                    Op::parse(),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    string_p(".pred"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, red, op, aligned, pred, p, _, a, b, _, c_op, c, _), span| {
                    ok!(BarrierCtaRedOpAlignedPred {
                        cta = cta,
                        red = red,
                        op = op,
                        aligned = aligned,
                        pred = pred,
                        p = p,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for BarCtaSync {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bar"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".sync"),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, cta, sync, a, b, _), span| {
                    ok!(BarCtaSync {
                        cta = cta,
                        sync = sync,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for BarCtaArrive {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bar"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".arrive"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, arrive, a, _, b, _), span| {
                    ok!(BarCtaArrive {
                        cta = cta,
                        arrive = arrive,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for BarCtaRedPopcU32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bar"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".red"),
                    string_p(".popc"),
                    string_p(".u32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, red, popc, u32, d, _, a, b, _, c_op, c, _), span| {
                    ok!(BarCtaRedPopcU32 {
                        cta = cta,
                        red = red,
                        popc = popc,
                        u32 = u32,
                        d = d,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for BarCtaRedOpPred {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bar"),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    string_p(".red"),
                    Op::parse(),
                    string_p(".pred"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    map(optional(exclamation_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cta, red, op, pred, p, _, a, b, _, c_op, c, _), span| {
                    ok!(BarCtaRedOpPred {
                        cta = cta,
                        red = red,
                        op = op,
                        pred = pred,
                        p = p,
                        a = a,
                        b = b,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }
}
