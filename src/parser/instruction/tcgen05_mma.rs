//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind   [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind   [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! ------------------------------------------------------------------
//! // 2. Floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .kind = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .cta_group      = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! ------------------------------------------------------------------
//! // 3. Convolution MMA for floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind.collector_usage [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1,   .cta_group::2 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 4. Activation Stationary MMA for floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize  = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! .kind            = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 5. Integer type:
//! tcgen05.mma.cta_group.kind::i8  [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8  [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group = { .cta_group::1,   .cta_group::2  };
//! ------------------------------------------------------------------
//! // 6. Convolution MMA for integer type:
//! tcgen05.mma.cta_group.kind::i8.collector_usage          [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2  };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };

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
    use crate::r#type::instruction::tcgen05_mma::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".kind::f8f6f4"), |_, _span| Kind::KindF8f6f4),
                map(string_p(".kind::tf32"), |_, _span| Kind::KindTf32),
                map(string_p(".kind::f16"), |_, _span| Kind::KindF16)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
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
                    cta_group,
                    kind,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    scale_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKind {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,
                        scale_input_d = scale_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKind1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    AddressOperand::parse(),
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
                    cta_group,
                    kind,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    scale_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKind1 {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,
                        scale_input_d = scale_input_d,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".kind::mxf8f6f4"), |_, _span| Kind::KindMxf8f6f4),
                map(string_p(".kind::mxf4nvf4"), |_, _span| Kind::KindMxf4nvf4),
                map(string_p(".kind::mxf4"), |_, _span| Kind::KindMxf4)
            )
        }
    }

    impl PtxParser for ScaleVectorsize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".scale_vec::1X"), |_, _span| {
                    ScaleVectorsize::ScaleVec1x
                }),
                map(string_p(".scale_vec::2X"), |_, _span| {
                    ScaleVectorsize::ScaleVec2x
                }),
                map(string_p(".scale_vec::4X"), |_, _span| {
                    ScaleVectorsize::ScaleVec4x
                }),
                map(string_p(".block16"), |_, _span| ScaleVectorsize::Block16),
                map(string_p(".block32"), |_, _span| ScaleVectorsize::Block32)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVectorsize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind,
                    block_scale,
                    scale_vectorsize,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    _,
                    scale_a_tmem,
                    _,
                    scale_b_tmem,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vectorsize = scale_vectorsize,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        scale_a_tmem = scale_a_tmem,
                        scale_b_tmem = scale_b_tmem,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVectorsize::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind,
                    block_scale,
                    scale_vectorsize,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    _,
                    scale_a_tmem,
                    _,
                    scale_b_tmem,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vectorsize = scale_vectorsize,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        scale_a_tmem = scale_a_tmem,
                        scale_b_tmem = scale_b_tmem,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_2::*;

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
                            let part0 = match stream.expect_strings(&["::a"])? {
                                0 => Buffer::A,
                                _ => unreachable!(),
                            };
                            let part1 = match stream.expect_strings(&[
                                "::discard*",
                                "::lastuse",
                                "::fill",
                                "::use",
                            ])? {
                                0 => Op::Discard,
                                1 => Op::Lastuse,
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

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".kind::f8f6f4"), |_, _span| Kind::KindF8f6f4),
                map(string_p(".kind::tf32"), |_, _span| Kind::KindTf32),
                map(string_p(".kind::f16"), |_, _span| Kind::KindF16)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindCollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
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
                    cta_group,
                    kind,
                    collector_usage,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    scale_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindCollectorUsage {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,
                        scale_input_d = scale_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    map(optional(string_p(".ashift")), |value, _| value.is_some()),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
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
                    cta_group,
                    kind,
                    ashift,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    scale_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindAshiftCollectorUsage {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        ashift = ashift,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,
                        scale_input_d = scale_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    string_p(".ashift"),
                    optional(CollectorUsage::parse()),
                    AddressOperand::parse(),
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
                    cta_group,
                    kind,
                    ashift,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    scale_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        ashift = ashift,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,
                        scale_input_d = scale_input_d,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_3::*;

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
                            let part0 = match stream.expect_strings(&["::a"])? {
                                0 => Buffer::A,
                                _ => unreachable!(),
                            };
                            let part1 = match stream.expect_strings(&[
                                "::discard*",
                                "::lastuse",
                                "::fill",
                                "::use",
                            ])? {
                                0 => Op::Discard,
                                1 => Op::Lastuse,
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

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".kind::mxf8f6f4"), |_, _span| Kind::KindMxf8f6f4),
                map(string_p(".kind::mxf4nvf4"), |_, _span| Kind::KindMxf4nvf4),
                map(string_p(".kind::mxf4"), |_, _span| Kind::KindMxf4)
            )
        }
    }

    impl PtxParser for ScaleVectorsize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".scale_vec::1X"), |_, _span| {
                    ScaleVectorsize::ScaleVec1x
                }),
                map(string_p(".scale_vec::2X"), |_, _span| {
                    ScaleVectorsize::ScaleVec2x
                }),
                map(string_p(".scale_vec::4X"), |_, _span| {
                    ScaleVectorsize::ScaleVec4x
                }),
                map(string_p(".block16"), |_, _span| ScaleVectorsize::Block16),
                map(string_p(".block32"), |_, _span| ScaleVectorsize::Block32)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVectorsize::parse()),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind,
                    block_scale,
                    scale_vectorsize,
                    collector_usage,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    _,
                    scale_a_tmem,
                    _,
                    scale_b_tmem,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vectorsize = scale_vectorsize,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        scale_a_tmem = scale_a_tmem,
                        scale_b_tmem = scale_b_tmem,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVectorsize::parse()),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind,
                    block_scale,
                    scale_vectorsize,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    _,
                    scale_a_tmem,
                    _,
                    scale_b_tmem,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
                        mma = mma,
                        cta_group = cta_group,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vectorsize = scale_vectorsize,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        scale_a_tmem = scale_a_tmem,
                        scale_b_tmem = scale_b_tmem,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    string_p(".kind::i8"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind_i8,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindI8 {
                        mma = mma,
                        cta_group = cta_group,
                        kind_i8 = kind_i8,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI81 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    string_p(".kind::i8"),
                    AddressOperand::parse(),
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
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind_i8,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindI81 {
                        mma = mma,
                        cta_group = cta_group,
                        kind_i8 = kind_i8,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }
}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_5::*;

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
                            let part0 = match stream.expect_strings(&["::a"])? {
                                0 => Buffer::A,
                                _ => unreachable!(),
                            };
                            let part1 = match stream.expect_strings(&[
                                "::discard*",
                                "::lastuse",
                                "::fill",
                                "::use",
                            ])? {
                                0 => Op::Discard,
                                1 => Op::Lastuse,
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

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8CollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    string_p(".kind::i8"),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind_i8,
                    collector_usage,
                    d_tmem,
                    _,
                    a_desc,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindI8CollectorUsage {
                        mma = mma,
                        cta_group = cta_group,
                        kind_i8 = kind_i8,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_desc = a_desc,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    string_p(".kind::i8"),
                    string_p(".ashift"),
                    optional(CollectorUsage::parse()),
                    AddressOperand::parse(),
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
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind_i8,
                    ashift,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage {
                        mma = mma,
                        cta_group = cta_group,
                        kind_i8 = kind_i8,
                        ashift = ashift,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".mma"),
                    CtaGroup::parse(),
                    string_p(".kind::i8"),
                    map(optional(string_p(".ashift")), |value, _| value.is_some()),
                    CollectorUsage::parse(),
                    AddressOperand::parse(),
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
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    mma,
                    cta_group,
                    kind_i8,
                    ashift,
                    collector_usage,
                    d_tmem,
                    _,
                    a_tmem,
                    _,
                    b_desc,
                    _,
                    idesc,
                    disable_output_lane,
                    _,
                    enable_input_d,
                    _,
                ),
                 span| {
                    ok!(Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 {
                        mma = mma,
                        cta_group = cta_group,
                        kind_i8 = kind_i8,
                        ashift = ashift,
                        collector_usage = collector_usage,
                        d_tmem = d_tmem,
                        a_tmem = a_tmem,
                        b_desc = b_desc,
                        idesc = idesc,
                        disable_output_lane = disable_output_lane,
                        enable_input_d = enable_input_d,

                    })
                },
            )
        }
    }
}
