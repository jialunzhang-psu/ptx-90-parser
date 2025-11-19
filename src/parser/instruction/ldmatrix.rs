//! Original PTX specification:
//!
//! ldmatrix.sync.aligned.shape.num{.trans}{.ss}.type r, [p];
//! ldmatrix.sync.aligned.m8n16.num{.ss}.dst_fmt.src_fmt        r, [p];
//! ldmatrix.sync.aligned.m16n16.num.trans{.ss}.dst_fmt.src_fmt r, [p];
//! .shape   = {.m8n8, .m16n16};
//! .num     = {.x1, .x2, .x4};
//! .ss      = {.shared, .shared::cta};
//! .type    = {.b16, .b8};
//! .dst_fmt = { .b8x16 };
//! .src_fmt = { .b6x16_p32, .b4x16_p64 };

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
    use crate::r#type::instruction::ldmatrix::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for DstFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".b8x16"), |_, _span| DstFmt::B8x16))
        }
    }

    impl PtxParser for Num {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".x1"), |_, _span| Num::X1),
                map(string_p(".x2"), |_, _span| Num::X2),
                map(string_p(".x4"), |_, _span| Num::X4)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n16"), |_, _span| Shape::M16n16),
                map(string_p(".m8n8"), |_, _span| Shape::M8n8)
            )
        }
    }

    impl PtxParser for SrcFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b6x16_p32"), |_, _span| SrcFmt::B6x16P32),
                map(string_p(".b4x16_p64"), |_, _span| SrcFmt::B4x16P64)
            )
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Type::B16),
                map(string_p(".b8"), |_, _span| Type::B8)
            )
        }
    }

    impl PtxParser for LdmatrixSyncAlignedShapeNumTransSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ldmatrix"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Num::parse(),
                    map(optional(string_p(".trans")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, aligned, shape, num, trans, ss, type_, r, _, p, _), span| {
                    ok!(LdmatrixSyncAlignedShapeNumTransSsType {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        num = num,
                        trans = trans,
                        ss = ss,
                        type_ = type_,
                        r = r,
                        p = p,

                    })
                },
            )
        }
    }

    impl PtxParser for LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ldmatrix"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m8n16"),
                    Num::parse(),
                    optional(Ss::parse()),
                    DstFmt::parse(),
                    SrcFmt::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, aligned, m8n16, num, ss, dst_fmt, src_fmt, r, _, p, _), span| {
                    ok!(LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt {
                        sync = sync,
                        aligned = aligned,
                        m8n16 = m8n16,
                        num = num,
                        ss = ss,
                        dst_fmt = dst_fmt,
                        src_fmt = src_fmt,
                        r = r,
                        p = p,

                    })
                },
            )
        }
    }

    impl PtxParser for LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("ldmatrix"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n16"),
                    Num::parse(),
                    string_p(".trans"),
                    optional(Ss::parse()),
                    DstFmt::parse(),
                    SrcFmt::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, aligned, m16n16, num, trans, ss, dst_fmt, src_fmt, r, _, p, _), span| {
                    ok!(LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt {
                        sync = sync,
                        aligned = aligned,
                        m16n16 = m16n16,
                        num = num,
                        trans = trans,
                        ss = ss,
                        dst_fmt = dst_fmt,
                        src_fmt = src_fmt,
                        r = r,
                        p = p,

                    })
                },
            )
        }
    }
}
