//! Original PTX specification:
//!
//! tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
//! tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! .comp  = { .r, .g, .b, .a };
//! .geom  = { .2d, .a2d, .cube, .acube };
//! .dtype = { .u32, .s32, .f32 };

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
    use crate::r#type::instruction::tld4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Comp {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".r"), |_, _span| Comp::R),
                map(string_p(".g"), |_, _span| Comp::G),
                map(string_p(".b"), |_, _span| Comp::B),
                map(string_p(".a"), |_, _span| Comp::A)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Dtype::U32),
                map(string_p(".s32"), |_, _span| Dtype::S32),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for Geom {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".acube"), |_, _span| Geom::Acube),
                map(string_p(".cube"), |_, _span| Geom::Cube),
                map(string_p(".a2d"), |_, _span| Geom::A2d),
                map(string_p(".2d"), |_, _span| Geom::_2d)
            )
        }
    }

    impl PtxParser for Tld4Comp2dV4DtypeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tld4"),
                    Comp::parse(),
                    string_p(".2d"),
                    string_p(".v4"),
                    Dtype::parse(),
                    string_p(".f32"),
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
                |(_, comp, _2d, v4, dtype, f32, d, p, _, a, e, f, _), span| {
                    ok!(Tld4Comp2dV4DtypeF32 {
                        comp = comp,
                        _2d = _2d,
                        v4 = v4,
                        dtype = dtype,
                        f32 = f32,
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

    impl PtxParser for Tld4CompGeomV4DtypeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tld4"),
                    Comp::parse(),
                    Geom::parse(),
                    string_p(".v4"),
                    Dtype::parse(),
                    string_p(".f32"),
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
                |(_, comp, geom, v4, dtype, f32, d, p, _, a, e, f, _), span| {
                    ok!(Tld4CompGeomV4DtypeF32 {
                        comp = comp,
                        geom = geom,
                        v4 = v4,
                        dtype = dtype,
                        f32 = f32,
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
}
