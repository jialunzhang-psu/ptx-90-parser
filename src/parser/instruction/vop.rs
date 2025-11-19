//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vop.dtype.atype.btype{.sat}       d, a{.asel}, b{.bsel};
//! vop.dtype.atype.btype{.sat}.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vop.dtype.atype.btype{.sat}  d.dsel, a{.asel}, b{.bsel}, c;
//! vop   = { vadd, vsub, vabsdiff, vmin, vmax };
//! .dtype = .atype = .btype = { .u32, .s32 };
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
    use crate::r#type::instruction::vop::section_0::*;

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

    impl PtxParser for Op2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op2::Add),
                map(string_p(".min"), |_, _span| Op2::Min),
                map(string_p(".max"), |_, _span| Op2::Max)
            )
        }
    }

    impl PtxParser for VaddDtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vadd"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VaddDtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VsubDtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vsub"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VsubDtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VabsdiffDtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vabsdiff"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VabsdiffDtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VminDtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmin"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VminDtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VmaxDtypeAtypeBtypeSat {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmax"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, dtype, atype, btype, sat, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VmaxDtypeAtypeBtypeSat {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VaddDtypeAtypeBtypeSatOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vadd"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VaddDtypeAtypeBtypeSatOp2 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VsubDtypeAtypeBtypeSatOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vsub"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VsubDtypeAtypeBtypeSatOp2 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VabsdiffDtypeAtypeBtypeSatOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vabsdiff"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VabsdiffDtypeAtypeBtypeSatOp2 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VminDtypeAtypeBtypeSatOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmin"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VminDtypeAtypeBtypeSatOp2 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VmaxDtypeAtypeBtypeSatOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmax"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VmaxDtypeAtypeBtypeSatOp2 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VaddDtypeAtypeBtypeSat1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vadd"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VaddDtypeAtypeBtypeSat1 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VsubDtypeAtypeBtypeSat1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vsub"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VsubDtypeAtypeBtypeSat1 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VabsdiffDtypeAtypeBtypeSat1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vabsdiff"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VabsdiffDtypeAtypeBtypeSat1 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VminDtypeAtypeBtypeSat1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmin"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VminDtypeAtypeBtypeSat1 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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

    impl PtxParser for VmaxDtypeAtypeBtypeSat1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vmax"),
                    Dtype::parse(),
                    Atype::parse(),
                    Btype::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, dtype, atype, btype, sat, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VmaxDtypeAtypeBtypeSat1 {
                        dtype = dtype,
                        atype = atype,
                        btype = btype,
                        sat = sat,
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
