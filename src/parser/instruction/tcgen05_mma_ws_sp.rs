//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.ws.sp.cta_group::1.kind{.collector_usage} [d-tmem],  a-desc,  b-desc,
//! [sp-meta-tmem] ,  idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.sp.cta_group::1.kind{.collector_usage} [d-tmem], [a-tmem], b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .kind = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! ------------------------------------------------------------------
//! // 2. Integer type:
//! tcgen05.mma.ws.sp.cta_group::1.kind::i8{.collector_usage} [d-tmem], a-desc, b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.sp.cta_group::1.kind::i8{.collector_usage} [d-tmem], [a-tmem], b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .collector_usage = { .collector::buffer::op };
//! ::buffer = { ::b0, ::b1, ::b2, ::b3 };
//! ::op   = { ::fill, ::use, ::lastuse, ::discard};

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
    use crate::r#type::instruction::tcgen05_mma_ws_sp::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".kind::f8f6f4"), |_, _span| Kind::KindF8f6f4),
                map(string_p(".kind::tf32"), |_, _span| Kind::KindTf32),
                map(string_p(".kind::f16"), |_, _span| Kind::KindF16)
            )
        }
    }

    impl PtxParser for Tcgen05MmaWsSpCtaGroup1KindCollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    string_p(".ws"),
                    string_p(".sp"),
                    string_p(".cta_group::1"),
                    Kind::parse(),
                    map(optional(string_p(".collector_usage")), |value, _| value
                        .is_some()),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    ws,
                    sp,
                    cta_group_1,
                    kind,
                    collector_usage,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta_tmem,
                    _,
                    idesc,
                    _,
                    enable_input_d,
                    zero_column_mask_desc,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaWsSpCtaGroup1KindCollectorUsage {
                        mma = mma,
                        ws = ws,
                        sp = sp,
                        cta_group_1 = cta_group_1,
                        kind = kind,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta_tmem = sp_meta_tmem,
                        idesc = idesc,
                        enable_input_d = enable_input_d,
                        zero_column_mask_desc = zero_column_mask_desc,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    string_p(".ws"),
                    string_p(".sp"),
                    string_p(".cta_group::1"),
                    Kind::parse(),
                    map(optional(string_p(".collector_usage")), |value, _| value
                        .is_some()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    ws,
                    sp,
                    cta_group_1,
                    kind,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    sp_meta_tmem,
                    _,
                    idesc,
                    _,
                    enable_input_d,
                    zero_column_mask_desc,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1 {
                        mma = mma,
                        ws = ws,
                        sp = sp,
                        cta_group_1 = cta_group_1,
                        kind = kind,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        sp_meta_tmem = sp_meta_tmem,
                        idesc = idesc,
                        enable_input_d = enable_input_d,
                        zero_column_mask_desc = zero_column_mask_desc,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma_ws_sp::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                |stream| {
                    stream.try_with_span(|stream| {
                        stream.with_partial_token_mode(|stream| {
                            stream.expect_string(".collector")?;
                            let part0 =
                                match stream.expect_strings(&["::b0", "::b1", "::b2", "::b3"])? {
                                    0 => Buffer::B0,
                                    1 => Buffer::B1,
                                    2 => Buffer::B2,
                                    3 => Buffer::B3,
                                    _ => unreachable!(),
                                };
                            let part1 = match stream.expect_strings(&[
                                "::lastuse",
                                "::discard",
                                "::fill",
                                "::use",
                            ])? {
                                0 => Op::Lastuse,
                                1 => Op::Discard,
                                2 => Op::Fill,
                                3 => Op::Use,
                                _ => unreachable!(),
                            };
                            Ok(((), part0, part1))
                        })
                    })
                },
                |(collector, buffer, op), _span| CollectorUsage::CollectorBufferOp(
                    collector, buffer, op
                )
            ))
        }
    }

    impl PtxParser for Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    string_p(".ws"),
                    string_p(".sp"),
                    string_p(".cta_group::1"),
                    string_p(".kind::i8"),
                    optional(CollectorUsage::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    ws,
                    sp,
                    cta_group_1,
                    kind_i8,
                    collector_usage,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    sp_meta_tmem,
                    _,
                    idesc,
                    _,
                    enable_input_d,
                    zero_column_mask_desc,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage {
                        mma = mma,
                        ws = ws,
                        sp = sp,
                        cta_group_1 = cta_group_1,
                        kind_i8 = kind_i8,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        sp_meta_tmem = sp_meta_tmem,
                        idesc = idesc,
                        enable_input_d = enable_input_d,
                        zero_column_mask_desc = zero_column_mask_desc,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    string_p(".ws"),
                    string_p(".sp"),
                    string_p(".cta_group::1"),
                    string_p(".kind::i8"),
                    optional(CollectorUsage::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    ws,
                    sp,
                    cta_group_1,
                    kind_i8,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    sp_meta_tmem,
                    _,
                    idesc,
                    _,
                    enable_input_d,
                    zero_column_mask_desc,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1 {
                        mma = mma,
                        ws = ws,
                        sp = sp,
                        cta_group_1 = cta_group_1,
                        kind_i8 = kind_i8,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        sp_meta_tmem = sp_meta_tmem,
                        idesc = idesc,
                        enable_input_d = enable_input_d,
                        zero_column_mask_desc = zero_column_mask_desc,

                    })
                },
            )
        }
    }
}
