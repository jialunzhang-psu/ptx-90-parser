//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! // mipmaps
//! tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms };
//! .dtype = { .u32, .s32, .f16,  .f32 };
//! .ctype = {       .s32, .f32 };          // .cube, .acube require .f32
//! // .2dms, .a2dms require .s32

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
    use crate::r#type::instruction::tex::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s32"), |_, _span| Ctype::S32),
                map(string_p(".f32"), |_, _span| Ctype::F32)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32),
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for Geom {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".acube"), |_, _span| Geom::Acube),
                map(string_p(".a2dms"), |_, _span| Geom::A2dms),
                map(string_p(".cube"), |_, _span| Geom::Cube),
                map(string_p(".2dms"), |_, _span| Geom::_2dms),
                map(string_p(".a1d"), |_, _span| Geom::A1d),
                map(string_p(".a2d"), |_, _span| Geom::A2d),
                map(string_p(".1d"), |_, _span| Geom::_1d),
                map(string_p(".2d"), |_, _span| Geom::_2d),
                map(string_p(".3d"), |_, _span| Geom::_3d)
            )
        }
    }

    impl PtxParser for TexGeomV4DtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler2::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, geom, v4, dtype, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexGeomV4DtypeCtype {
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexGeomV4DtypeCtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, geom, v4, dtype, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexGeomV4DtypeCtype1 {
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexGeomV2F16x2Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    Geom::parse(),
                    string_p(".v2"),
                    string_p(".f16x2"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler2::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, geom, v2, f16x2, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexGeomV2F16x2Ctype {
                        geom = geom,
                        v2 = v2,
                        f16x2 = f16x2,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexGeomV2F16x2Ctype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    Geom::parse(),
                    string_p(".v2"),
                    string_p(".f16x2"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, geom, v2, f16x2, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexGeomV2F16x2Ctype1 {
                        geom = geom,
                        v2 = v2,
                        f16x2 = f16x2,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexBaseGeomV4DtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".base"),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, base, geom, v4, dtype, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexBaseGeomV4DtypeCtype {
                        base = base,
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexLevelGeomV4DtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".level"),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, level, geom, v4, dtype, ctype, d, p, _, a, _, lod, e, f, _), span| {
                    ok!(TexLevelGeomV4DtypeCtype {
                        level = level,
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        lod = lod,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexGradGeomV4DtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".grad"),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, grad, geom, v4, dtype, ctype, d, p, _, a, _, dpdx, _, dpdy, e, f, _), span| {
                    ok!(TexGradGeomV4DtypeCtype {
                        grad = grad,
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        dpdx = dpdx,
                        dpdy = dpdy,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexBaseGeomV2F16x2Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".base"),
                    Geom::parse(),
                    string_p(".v2"),
                    string_p(".f16x2"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, base, geom, v2, f16x2, ctype, d, p, _, a, e, f, _), span| {
                    ok!(TexBaseGeomV2F16x2Ctype {
                        base = base,
                        geom = geom,
                        v2 = v2,
                        f16x2 = f16x2,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexLevelGeomV2F16x2Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".level"),
                    Geom::parse(),
                    string_p(".v2"),
                    string_p(".f16x2"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, level, geom, v2, f16x2, ctype, d, p, _, a, _, lod, e, f, _), span| {
                    ok!(TexLevelGeomV2F16x2Ctype {
                        level = level,
                        geom = geom,
                        v2 = v2,
                        f16x2 = f16x2,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        lod = lod,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }

    impl PtxParser for TexGradGeomV2F16x2Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tex"),
                    string_p(".grad"),
                    Geom::parse(),
                    string_p(".v2"),
                    string_p(".f16x2"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(pipe_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    comma_p(),
                    TexHandler3Optional::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, grad, geom, v2, f16x2, ctype, d, p, _, a, _, dpdx, _, dpdy, e, f, _), span| {
                    ok!(TexGradGeomV2F16x2Ctype {
                        grad = grad,
                        geom = geom,
                        v2 = v2,
                        f16x2 = f16x2,
                        ctype = ctype,
                        d = d,
                        p = p,
                        a = a,
                        dpdx = dpdx,
                        dpdy = dpdy,
                        e = e,
                        f = f,

                    })
                },
            )
        }
    }
}
