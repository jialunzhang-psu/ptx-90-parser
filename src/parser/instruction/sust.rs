//! Original PTX specification:
//!
//! sust.b.dim{.cop}.vec.ctype{.mode} [a, b], c;  // unformatted
//! sust.p.dim.vec.b32{.mode}       [a, b], c;  // formatted
//! sust.b.adim{.cop}.vec.ctype{.mode}   [a, b], c;  // unformatted
//! .cop   = { .wb, .cg, .cs, .wt };                     // cache operation
//! .vec   = { none, .v2, .v4 };
//! .ctype = { .b8 , .b16, .b32, .b64 };
//! .mode  = { .trap, .clamp, .zero };
//! .dim   = { .1d, .2d, .3d };
//! .adim  = { .a1d, .a2d };

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
    use crate::r#type::instruction::sust::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Adim {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".a1d"), |_, _span| Adim::A1d),
                map(string_p(".a2d"), |_, _span| Adim::A2d)
            )
        }
    }

    impl PtxParser for Cop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".wb"), |_, _span| Cop::Wb),
                map(string_p(".cg"), |_, _span| Cop::Cg),
                map(string_p(".cs"), |_, _span| Cop::Cs),
                map(string_p(".wt"), |_, _span| Cop::Wt)
            )
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Ctype::B16),
                map(string_p(".b32"), |_, _span| Ctype::B32),
                map(string_p(".b64"), |_, _span| Ctype::B64),
                map(string_p(".b8"), |_, _span| Ctype::B8)
            )
        }
    }

    impl PtxParser for Dim {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".1d"), |_, _span| Dim::_1d),
                map(string_p(".2d"), |_, _span| Dim::_2d),
                map(string_p(".3d"), |_, _span| Dim::_3d)
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

    impl PtxParser for SustBDimCopVecCtypeMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sust"),
                    string_p(".b"),
                    Dim::parse(),
                    optional(Cop::parse()),
                    Vec::parse(),
                    Ctype::parse(),
                    optional(Mode::parse()),
                    TexHandler2::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b, dim, cop, vec, ctype, mode, a, _, c, _), span| {
                    ok!(SustBDimCopVecCtypeMode {
                        b = b,
                        dim = dim,
                        cop = cop,
                        vec = vec,
                        ctype = ctype,
                        mode = mode,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SustPDimVecB32Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sust"),
                    string_p(".p"),
                    Dim::parse(),
                    Vec::parse(),
                    string_p(".b32"),
                    optional(Mode::parse()),
                    TexHandler2::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, p, dim, vec, b32, mode, a, _, c, _), span| {
                    ok!(SustPDimVecB32Mode {
                        p = p,
                        dim = dim,
                        vec = vec,
                        b32 = b32,
                        mode = mode,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for SustBAdimCopVecCtypeMode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("sust"),
                    string_p(".b"),
                    Adim::parse(),
                    optional(Cop::parse()),
                    Vec::parse(),
                    Ctype::parse(),
                    optional(Mode::parse()),
                    TexHandler2::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, b, adim, cop, vec, ctype, mode, a, _, c, _), span| {
                    ok!(SustBAdimCopVecCtypeMode {
                        b = b,
                        adim = adim,
                        cop = cop,
                        vec = vec,
                        ctype = ctype,
                        mode = mode,
                        a = a,
                        c = c,

                    })
                },
            )
        }
    }
}
