//! Original PTX specification:
//!
//! // 32-bit scalar operation
//! vmad.dtype.atype.btype{.sat}{.scale}     d, {-}a{.asel}, {-}b{.bsel},
//! {-}c;
//! vmad.dtype.atype.btype.po{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
//! .scale = { .shr7, .shr15 };

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
    use crate::r#type::instruction::vmad::section_0::*;

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

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Btype::U32),
                map(string_p(".s32"), |_, _span| Btype::S32)
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

    impl PtxParser for Scale {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shr15"), |_, _span| Scale::Shr15),
                map(string_p(".shr7"), |_, _span| Scale::Shr7)
            )
        }
    }

    impl PtxParser for VmadDtypeAtypeBtypeSatScale {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmad"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    optional(Scale::parse()),
                    GeneralOperand::parse(),
                    comma_p(),
                    map(optional(minus_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    map(optional(minus_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    map(optional(minus_p()), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    dtype,
                    atype,
                    btype,
                    sat,
                    scale,
                    d,
                    _,
                    a_op,
                    a,
                    asel,
                    _,
                    b_op,
                    b,
                    bsel,
                    _,
                    c_op,
                    c,
                    _,
                ),
                 span| {
                    ok!(VmadDtypeAtypeBtypeSatScale {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
                        scale = scale,
                        d = d,
                        a_op = a_op,
                        a = a,
                        asel = asel,
                        b_op = b_op,
                        b = b,
                        bsel = bsel,
                        c_op = c_op,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for VmadDtypeAtypeBtypePoSatScale {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmad"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".po"),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    optional(Scale::parse()),
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
                |(_, dtype, atype, btype, po, sat, scale, d, _, a, asel, _, b, bsel, _, c, _),
                 span| {
                    ok!(VmadDtypeAtypeBtypePoSatScale {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        po = po,
                        sat = sat,
                        scale = scale,
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
}
