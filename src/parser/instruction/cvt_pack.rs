//! Original PTX specification:
//!
//! cvt.pack.sat.convertType.abType  d, a, b;
//! .convertType  = { .u16, .s16 };
//! .abType       = { .s32 };
//! ----------------------------------------------------------------
//! cvt.pack.sat.convertType.abType.cType  d, a, b, c;
//! .convertType  = { .u2, .s2, .u4, .s4, .u8, .s8 };
//! .abType       = { .s32 };
//! .cType        = { .b32 };

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
    use crate::r#type::instruction::cvt_pack::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".s32"), |_, _span| Abtype::S32))
        }
    }

    impl PtxParser for Converttype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16"), |_, _span| Converttype::U16),
                map(string_p(".s16"), |_, _span| Converttype::S16)
            )
        }
    }

    impl PtxParser for CvtPackSatConverttypeAbtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".pack"),
                    string_p(".sat"),
                    Converttype::parse(),
                    Abtype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, pack, sat, converttype, abtype, d, _, a, _, b, _), span| {
                    ok!(CvtPackSatConverttypeAbtype {
                        pack = pack,
                        sat = sat,
                        converttype = converttype,
                        abtype = abtype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cvt_pack::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".s32"), |_, _span| Abtype::S32))
        }
    }

    impl PtxParser for Converttype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u2"), |_, _span| Converttype::U2),
                map(string_p(".s2"), |_, _span| Converttype::S2),
                map(string_p(".u4"), |_, _span| Converttype::U4),
                map(string_p(".s4"), |_, _span| Converttype::S4),
                map(string_p(".u8"), |_, _span| Converttype::U8),
                map(string_p(".s8"), |_, _span| Converttype::S8)
            )
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".b32"), |_, _span| Ctype::B32))
        }
    }

    impl PtxParser for CvtPackSatConverttypeAbtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvt"),
                    string_p(".pack"),
                    string_p(".sat"),
                    Converttype::parse(),
                    Abtype::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, pack, sat, converttype, abtype, ctype, d, _, a, _, b, _, c, _), span| {
                    ok!(CvtPackSatConverttypeAbtypeCtype {
                        pack = pack,
                        sat = sat,
                        converttype = converttype,
                        abtype = abtype,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}
