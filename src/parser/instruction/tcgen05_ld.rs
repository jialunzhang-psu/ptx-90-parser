//! Original PTX specification:
//!
//! // Base load instruction:
//! tcgen05.ld.sync.aligned.shape1.num{.pack}.b32    r, [taddr];
//! tcgen05.ld.sync.aligned.shape2.num{.pack}.b32    r, [taddr], immHalfSplitoff;
//! .shape1 = { .16x64b, .16x128b, .16x256b, .32x32b };
//! .shape2 = { .16x32bx2 };
//! .num    = { .x1, .x2, .x4, .x8, .x16, .x32, .x64, .x128 };
//! .pack   = { .pack::16b };
//! // Floating point type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp{.abs}{.NaN}.f32 r, redval, [taddr], immHalfSplitoff;
//! // Integer type load along with reduction :
//! tcgen05.ld.red.sync.aligned.shape3.num.redOp.type r, redval, [taddr];
//! tcgen05.ld.red.sync.aligned.shape4.num.redOp.type r, redval, [taddr], immHalfSplitoff;
//! .shape3 = { .32x32b   };
//! .shape4 = { .16x32bx2 };
//! .redOp  = { .min, .max };
//! .type   = { .u32, .s32 };

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
    use crate::r#type::instruction::tcgen05_ld::section_0::*;

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

    impl PtxParser for Pack {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".pack::16b"), |_, _span| Pack::Pack16b))
        }
    }

    impl PtxParser for Redop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".min"), |_, _span| Redop::Min),
                map(string_p(".max"), |_, _span| Redop::Max)
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

    impl PtxParser for Shape3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".32x32b"), |_, _span| Shape3::_32x32b))
        }
    }

    impl PtxParser for Shape4 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".16x32bx2"), |_, _span| Shape4::_16x32bx2))
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

    impl PtxParser for Tcgen05LdSyncAlignedShape1NumPackB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape1::parse(),
                    Num::parse(),
                    optional(Pack::parse()),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, ld, sync, aligned, shape1, num, pack, b32, r, _, taddr, _), span| {
                    ok!(Tcgen05LdSyncAlignedShape1NumPackB32 {
                        ld = ld,
                        sync = sync,
                        aligned = aligned,
                        shape1 = shape1,
                        num = num,
                        pack = pack,
                        b32 = b32,
                        r = r,
                        taddr = taddr,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05LdSyncAlignedShape2NumPackB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape2::parse(),
                    Num::parse(),
                    optional(Pack::parse()),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    ld,
                    sync,
                    aligned,
                    shape2,
                    num,
                    pack,
                    b32,
                    r,
                    _,
                    taddr,
                    _,
                    immhalfsplitoff,
                    _,
                ),
                 span| {
                    ok!(Tcgen05LdSyncAlignedShape2NumPackB32 {
                        ld = ld,
                        sync = sync,
                        aligned = aligned,
                        shape2 = shape2,
                        num = num,
                        pack = pack,
                        b32 = b32,
                        r = r,
                        taddr = taddr,
                        immhalfsplitoff = immhalfsplitoff,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".red"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape3::parse(),
                    Num::parse(),
                    Redop::parse(),
                    map(optional(string_p(".abs")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    ld,
                    red,
                    sync,
                    aligned,
                    shape3,
                    num,
                    redop,
                    abs,
                    nan,
                    f32,
                    r,
                    _,
                    redval,
                    _,
                    taddr,
                    _,
                ),
                 span| {
                    ok!(Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 {
                        ld = ld,
                        red = red,
                        sync = sync,
                        aligned = aligned,
                        shape3 = shape3,
                        num = num,
                        redop = redop,
                        abs = abs,
                        nan = nan,
                        f32 = f32,
                        r = r,
                        redval = redval,
                        taddr = taddr,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".red"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape4::parse(),
                    Num::parse(),
                    Redop::parse(),
                    map(optional(string_p(".abs")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    ld,
                    red,
                    sync,
                    aligned,
                    shape4,
                    num,
                    redop,
                    abs,
                    nan,
                    f32,
                    r,
                    _,
                    redval,
                    _,
                    taddr,
                    _,
                    immhalfsplitoff,
                    _,
                ),
                 span| {
                    ok!(Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 {
                        ld = ld,
                        red = red,
                        sync = sync,
                        aligned = aligned,
                        shape4 = shape4,
                        num = num,
                        redop = redop,
                        abs = abs,
                        nan = nan,
                        f32 = f32,
                        r = r,
                        redval = redval,
                        taddr = taddr,
                        immhalfsplitoff = immhalfsplitoff,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05LdRedSyncAlignedShape3NumRedopType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".red"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape3::parse(),
                    Num::parse(),
                    Redop::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    ld,
                    red,
                    sync,
                    aligned,
                    shape3,
                    num,
                    redop,
                    type_,
                    r,
                    _,
                    redval,
                    _,
                    taddr,
                    _,
                ),
                 span| {
                    ok!(Tcgen05LdRedSyncAlignedShape3NumRedopType {
                        ld = ld,
                        red = red,
                        sync = sync,
                        aligned = aligned,
                        shape3 = shape3,
                        num = num,
                        redop = redop,
                        type_ = type_,
                        r = r,
                        redval = redval,
                        taddr = taddr,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05LdRedSyncAlignedShape4NumRedopType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".ld"),
                    string_p(".red"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape4::parse(),
                    Num::parse(),
                    Redop::parse(),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    ld,
                    red,
                    sync,
                    aligned,
                    shape4,
                    num,
                    redop,
                    type_,
                    r,
                    _,
                    redval,
                    _,
                    taddr,
                    _,
                    immhalfsplitoff,
                    _,
                ),
                 span| {
                    ok!(Tcgen05LdRedSyncAlignedShape4NumRedopType {
                        ld = ld,
                        red = red,
                        sync = sync,
                        aligned = aligned,
                        shape4 = shape4,
                        num = num,
                        redop = redop,
                        type_ = type_,
                        r = r,
                        redval = redval,
                        taddr = taddr,
                        immhalfsplitoff = immhalfsplitoff,

                    })
                },
            )
        }
    }
}
