//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.u32{.sat}.mode       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.u32{.sat}.mode.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.u32{.sat}.mode  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vshl, vshr };
//! .dtype = .atype = { .u32, .s32 };
//! .mode  = { .clamp, .wrap };
//! .dsel  = .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .op2   = { .add, .min, .max };

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
    use crate::r#type::instruction::vsh::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Asel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b0"), |_, _span| Asel::B0),
                map(string_p(".b1"), |_, _span| Asel::B1),
                map(string_p(".b2"), |_, _span| Asel::B2),
                map(string_p(".b3"), |_, _span| Asel::B3),
                map(string_p(".h0"), |_, _span| Asel::H0),
                map(string_p(".h1"), |_, _span| Asel::H1)
            )
        }
    }

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Atype::U32),
                map(string_p(".s32"), |_, _span| Atype::S32)
            )
        }
    }

    impl PtxParser for Bsel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b0"), |_, _span| Bsel::B0),
                map(string_p(".b1"), |_, _span| Bsel::B1),
                map(string_p(".b2"), |_, _span| Bsel::B2),
                map(string_p(".b3"), |_, _span| Bsel::B3),
                map(string_p(".h0"), |_, _span| Bsel::H0),
                map(string_p(".h1"), |_, _span| Bsel::H1)
            )
        }
    }

    impl PtxParser for Dsel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b0"), |_, _span| Dsel::B0),
                map(string_p(".b1"), |_, _span| Dsel::B1),
                map(string_p(".b2"), |_, _span| Dsel::B2),
                map(string_p(".b3"), |_, _span| Dsel::B3),
                map(string_p(".h0"), |_, _span| Dsel::H0),
                map(string_p(".h1"), |_, _span| Dsel::H1)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32)
            )
        }
    }

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".clamp"), |_, _span| Mode::Clamp),
                map(string_p(".wrap"), |_, _span| Mode::Wrap)
            )
        }
    }

    impl PtxParser for Op2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op2::Add),
                map(string_p(".min"), |_, _span| Op2::Min),
                map(string_p(".max"), |_, _span| Op2::Max)
            )
        }
    }

    impl PtxParser for VshlDtypeAtypeU32SatMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshl"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VshlDtypeAtypeU32SatMode {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        d = d,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,

                    })
                },
            )
        }
    }

    impl PtxParser for VshrDtypeAtypeU32SatMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshr"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VshrDtypeAtypeU32SatMode {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        d = d,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,

                    })
                },
            )
        }
    }

    impl PtxParser for VshlDtypeAtypeU32SatModeOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshl"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    Op2::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, op2, d, _, a, asel, _, b, bsel, _, c, _),
                 span| {
                    ok!(VshlDtypeAtypeU32SatModeOp2 {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        op2 = op2,
                        d = d,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for VshrDtypeAtypeU32SatModeOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshr"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    Op2::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, op2, d, _, a, asel, _, b, bsel, _, c, _),
                 span| {
                    ok!(VshrDtypeAtypeU32SatModeOp2 {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        op2 = op2,
                        d = d,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for VshlDtypeAtypeU32SatMode1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshl"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    GeneralOperand::parse(),
                    Dsel::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, d, dsel, _, a, asel, _, b, bsel, _, c, _),
                 span| {
                    ok!(VshlDtypeAtypeU32SatMode1 {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        d = d,
                        dsel = dsel,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for VshrDtypeAtypeU32SatMode1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vshr"),
                    Dtype::parse(),
                    Atype::parse(),
                    string_p(".u32"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    Mode::parse(),
                    GeneralOperand::parse(),
                    Dsel::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dtype, atype, u32, sat, mode, d, dsel, _, a, asel, _, b, bsel, _, c, _),
                 span| {
                    ok!(VshrDtypeAtypeU32SatMode1 {
                        dtype = dtype,
                        atype = atype,
                        u32 = u32,
                        sat = sat,
                        mode = mode,
                        d = d,
                        dsel = dsel,
                        a = a,
                        asel = asel,
                        b = b,
                        bsel = bsel,
                        c = c,

                    })
                },
            )
        }
    }
}
