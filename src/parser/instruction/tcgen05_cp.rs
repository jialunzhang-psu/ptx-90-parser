//! Original PTX specification:
//!
//! tcgen05.cp.cta_group.shape{.multicast}{.dst_src_fmt} [taddr], s-desc;
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! .dst_src_fmt   = { .b8x16.b6x16_p32, .b8x16.b4x16_p64 };
//! .shape     = { .128x256b, .4x256b, .128x128b, .64x128b**, .32x128b*** };
//! .multicast = { .warpx2::02_13** , .warpx2::01_23**, .warpx4*** };

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
    use crate::r#type::instruction::tcgen05_cp::section_0::*;

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

    impl PtxParser for DstSrcFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b8x16.b6x16_p32"), |_, _span| {
                    DstSrcFmt::B8x16B6x16P32
                }),
                map(string_p(".b8x16.b4x16_p64"), |_, _span| {
                    DstSrcFmt::B8x16B4x16P64
                })
            )
        }
    }

    impl PtxParser for Multicast {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".warpx2::02_13**"), |_, _span| {
                    Multicast::Warpx20213
                }),
                map(string_p(".warpx2::01_23**"), |_, _span| {
                    Multicast::Warpx20123
                }),
                map(string_p(".warpx4***"), |_, _span| Multicast::Warpx4)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".32x128b***"), |_, _span| Shape::_32x128b),
                map(string_p(".64x128b**"), |_, _span| Shape::_64x128b),
                map(string_p(".128x256b"), |_, _span| Shape::_128x256b),
                map(string_p(".128x128b"), |_, _span| Shape::_128x128b),
                map(string_p(".4x256b"), |_, _span| Shape::_4x256b)
            )
        }
    }

    impl PtxParser for Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".cp"),
                    CtaGroup::parse(),
                    Shape::parse(),
                    optional(Multicast::parse()),
                    optional(DstSrcFmt::parse()),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cp, cta_group, shape, multicast, dst_src_fmt, taddr, _, s_desc, _), span| {
                    ok!(Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
                        cp = cp,
                        cta_group = cta_group,
                        shape = shape,
                        multicast = multicast,
                        dst_src_fmt = dst_src_fmt,
                        taddr = taddr,
                        s_desc = s_desc,

                    })
                },
            )
        }
    }
}
