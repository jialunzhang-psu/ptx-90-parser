//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype{.mode}  d, [a, b];  // unformatted
//!
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d };
//! .cop   = { .ca, .cg, .cs, .cv };               // cache operation
//! .vec   = { none, .v2, .v4 };
//! .dtype = { .b8 , .b16, .b32, .b64 };
//! .mode = { .trap, .clamp, .zero };

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
    use crate::r#type::instruction::suld::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Cop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".ca"), |_, _span| Cop::Ca),
                map(string_p(".cg"), |_, _span| Cop::Cg),
                map(string_p(".cs"), |_, _span| Cop::Cs),
                map(string_p(".cv"), |_, _span| Cop::Cv)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Dtype::B16),
                map(string_p(".b32"), |_, _span| Dtype::B32),
                map(string_p(".b64"), |_, _span| Dtype::B64),
                map(string_p(".b8"), |_, _span| Dtype::B8)
            )
        }
    }

    impl PtxParser for Geom {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".a1d"), |_, _span| Geom::A1d),
                map(string_p(".a2d"), |_, _span| Geom::A2d),
                map(string_p(".1d"), |_, _span| Geom::_1d),
                map(string_p(".2d"), |_, _span| Geom::_2d),
                map(string_p(".3d"), |_, _span| Geom::_3d)
            )
        }
    }

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".clamp"), |_, _span| Mode::Clamp),
                map(string_p(".trap"), |_, _span| Mode::Trap),
                map(string_p(".zero"), |_, _span| Mode::Zero)
            )
        }
    }

    impl PtxParser for Vec {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p("none"), |_, _span| Vec::None),
                map(string_p(".v2"), |_, _span| Vec::V2),
                map(string_p(".v4"), |_, _span| Vec::V4)
            )
        }
    }

    impl PtxParser for SuldBGeomCopVecDtypeMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("suld"),
                    string_p(".b"),
                    Geom::parse(),
                    optional(Cop::parse()),
                    Vec::parse(),
                    Dtype::parse(),
                    optional(Mode::parse()),
                    GeneralOperand::parse(),
                    comma_p(),
                    TexHandler2::parse(),
                    semicolon_p()
                ),
                |(_, b, geom, cop, vec, dtype, mode, d, _, a, _), span| {
                    ok!(SuldBGeomCopVecDtypeMode {
                        b = b,
                        geom = geom,
                        cop = cop,
                        vec = vec,
                        dtype = dtype,
                        mode = mode,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
