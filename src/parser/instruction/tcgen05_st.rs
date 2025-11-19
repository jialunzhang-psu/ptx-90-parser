//! Original PTX specification:
//!
//! tcgen05.st.sync.aligned.shape1.num{.unpack}.b32    [taddr], r;
//! tcgen05.st.sync.aligned.shape2.num{.unpack}.b32    [taddr], immHalfSplitoff, r;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .unpack = { .unpack::16b };

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
    use crate::r#type::instruction::tcgen05_st::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Num {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".x128"), |_, _span| Num::X128),
                map(string_p(".x16"), |_, _span| Num::X16),
                map(string_p(".x32"), |_, _span| Num::X32),
                map(string_p(".x64"), |_, _span| Num::X64),
                map(string_p(".x1"), |_, _span| Num::X1),
                map(string_p(".x2"), |_, _span| Num::X2),
                map(string_p(".x4"), |_, _span| Num::X4),
                map(string_p(".x8"), |_, _span| Num::X8)
            )
        }
    }

    impl PtxParser for Shape1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".16x128b"), |_, _span| Shape1::_16x128b),
                map(string_p(".16x256b"), |_, _span| Shape1::_16x256b),
                map(string_p(".16x64b"), |_, _span| Shape1::_16x64b),
                map(string_p(".32x32b"), |_, _span| Shape1::_32x32b)
            )
        }
    }

    impl PtxParser for Shape2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".16x32bx2"), |_, _span| Shape2::_16x32bx2))
        }
    }

    impl PtxParser for Unpack {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".unpack::16b"), |_, _span| Unpack::Unpack16b))
        }
    }

    impl PtxParser for Tcgen05StSyncAlignedShape1NumUnpackB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".st"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape1::parse(),
                    Num::parse(),
                    optional(Unpack::parse()),
                    string_p(".b32"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, st, sync, aligned, shape1, num, unpack, b32, taddr, _, r, _), span| {
                    ok!(Tcgen05StSyncAlignedShape1NumUnpackB32 {
                        st = st,
                        sync = sync,
                        aligned = aligned,
                        shape1 = shape1,
                        num = num,
                        unpack = unpack,
                        b32 = b32,
                        taddr = taddr,
                        r = r,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05StSyncAlignedShape2NumUnpackB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".st"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape2::parse(),
                    Num::parse(),
                    optional(Unpack::parse()),
                    string_p(".b32"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    st,
                    sync,
                    aligned,
                    shape2,
                    num,
                    unpack,
                    b32,
                    taddr,
                    _,
                    immhalfsplitoff,
                    _,
                    r,
                    _,
                ),
                 span| {
                    ok!(Tcgen05StSyncAlignedShape2NumUnpackB32 {
                        st = st,
                        sync = sync,
                        aligned = aligned,
                        shape2 = shape2,
                        num = num,
                        unpack = unpack,
                        b32 = b32,
                        taddr = taddr,
                        immhalfsplitoff = immhalfsplitoff,
                        r = r,

                    })
                },
            )
        }
    }
}
