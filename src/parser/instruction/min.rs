//! Original PTX specification:
//!
//! min.atype         d, a, b;
//! min{.relu}.btype  d, a, b;
//! .atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 };
//! .btype = { .s16x2, .s32 };
//!
//! min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
//! min{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
//! min.f64                            d, a, b;
//!
//! min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
//! min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16           d, a, b;
//! min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;

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
    use crate::r#type::instruction::min::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u16x2"), |_, _span| Atype::U16x2),
                map(string_p(".u16"), |_, _span| Atype::U16),
                map(string_p(".u32"), |_, _span| Atype::U32),
                map(string_p(".u64"), |_, _span| Atype::U64),
                map(string_p(".s16"), |_, _span| Atype::S16),
                map(string_p(".s64"), |_, _span| Atype::S64)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s16x2"), |_, _span| Btype::S16x2),
                map(string_p(".s32"), |_, _span| Btype::S32)
            )
        }
    }

    impl PtxParser for MinAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, atype, d, _, a, _, b, _), span| {
                    ok!(MinAtype {
                        atype = atype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinReluBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, relu, btype, d, _, a, _, b, _), span| {
                    ok!(MinReluBtype {
                        relu = relu,
                        btype = btype,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinFtzNanXorsignAbsF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".xorsign.abs")), |value, _| value
                        .is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, nan, xorsign_abs, f32, d, _, a, _, b, _), span| {
                    ok!(MinFtzNanXorsignAbsF32 {
                        ftz = ftz,
                        nan = nan,
                        xorsign_abs = xorsign_abs,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinFtzNanAbsF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".abs")), |value, _| value.is_some()),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, nan, abs, f32, d, _, a, _, b, _, c, _), span| {
                    ok!(MinFtzNanAbsF32 {
                        ftz = ftz,
                        nan = nan,
                        abs = abs,
                        f32 = f32,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MinF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, f64, d, _, a, _, b, _), span| {
                    ok!(MinF64 {
                        f64 = f64,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinFtzNanXorsignAbsF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".xorsign.abs")), |value, _| value
                        .is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, nan, xorsign_abs, f16, d, _, a, _, b, _), span| {
                    ok!(MinFtzNanXorsignAbsF16 {
                        ftz = ftz,
                        nan = nan,
                        xorsign_abs = xorsign_abs,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinFtzNanXorsignAbsF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".xorsign.abs")), |value, _| value
                        .is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, ftz, nan, xorsign_abs, f16x2, d, _, a, _, b, _), span| {
                    ok!(MinFtzNanXorsignAbsF16x2 {
                        ftz = ftz,
                        nan = nan,
                        xorsign_abs = xorsign_abs,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinNanXorsignAbsBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".xorsign.abs")), |value, _| value
                        .is_some()),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, nan, xorsign_abs, bf16, d, _, a, _, b, _), span| {
                    ok!(MinNanXorsignAbsBf16 {
                        nan = nan,
                        xorsign_abs = xorsign_abs,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }

    impl PtxParser for MinNanXorsignAbsBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("min"),
                    map(optional(string_p(".NaN")), |value, _| value.is_some()),
                    map(optional(string_p(".xorsign.abs")), |value, _| value
                        .is_some()),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, nan, xorsign_abs, bf16x2, d, _, a, _, b, _), span| {
                    ok!(MinNanXorsignAbsBf16x2 {
                        nan = nan,
                        xorsign_abs = xorsign_abs,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
