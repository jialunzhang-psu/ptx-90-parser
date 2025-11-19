//! Original PTX specification:
//!
//! fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
//! fma.rnd{.ftz}.f32x2      d, a, b, c;
//! fma.rnd.f64              d, a, b, c;
//! .rnd = { .rn, .rz, .rm, .rp };
//! ---------------------------------------------
//! fma.rnd{.ftz}{.sat}.f16     d, a, b, c;
//! fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
//! fma.rnd{.ftz}.relu.f16      d, a, b, c;
//! fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
//! fma.rnd{.relu}.bf16         d, a, b, c;
//! fma.rnd{.relu}.bf16x2       d, a, b, c;
//! fma.rnd.oob{.relu}.type     d, a, b, c;
//! .rnd = { .rn };
//! ---------------------------------------------
//! fma.rnd{.sat}.f32.abtype  d, a, b, c;
//! .abtype = { .f16, .bf16};
//! .rnd    = { .rn, .rz, .rm, .rp };

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
    use crate::r#type::instruction::fma::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Rnd::Rn),
                map(string_p(".rz"), |_, _span| Rnd::Rz),
                map(string_p(".rm"), |_, _span| Rnd::Rm),
                map(string_p(".rp"), |_, _span| Rnd::Rp)
            )
        }
    }

    impl PtxParser for FmaRndFtzSatF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
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
                |(_, rnd, ftz, sat, f32, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzSatF32 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
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

    impl PtxParser for FmaRndFtzF32x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".f32x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, f32x2, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzF32x2 {
                        rnd = rnd,
                        ftz = ftz,
                        f32x2 = f32x2,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndF64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, f64, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndF64 {
                        rnd = rnd,
                        f64 = f64,
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

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::fma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".rn"), |_, _span| Rnd::Rn))
        }
    }

    impl PtxParser for FmaRndFtzSatF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, sat, f16, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzSatF16 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndFtzSatF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, sat, f16x2, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzSatF16x2 {
                        rnd = rnd,
                        ftz = ftz,
                        sat = sat,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndFtzReluF16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".relu"),
                    string_p(".f16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, relu, f16, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzReluF16 {
                        rnd = rnd,
                        ftz = ftz,
                        relu = relu,
                        f16 = f16,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndFtzReluF16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".ftz")), |value, _| value.is_some()),
                    string_p(".relu"),
                    string_p(".f16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, ftz, relu, f16x2, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndFtzReluF16x2 {
                        rnd = rnd,
                        ftz = ftz,
                        relu = relu,
                        f16x2 = f16x2,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndReluBf16 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".bf16"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, relu, bf16, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndReluBf16 {
                        rnd = rnd,
                        relu = relu,
                        bf16 = bf16,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndReluBf16x2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".bf16x2"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, relu, bf16x2, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndReluBf16x2 {
                        rnd = rnd,
                        relu = relu,
                        bf16x2 = bf16x2,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for FmaRndOobReluType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    string_p(".oob"),
                    map(optional(string_p(".relu")), |value, _| value.is_some()),
                    string_p(".type"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, oob, relu, type_, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndOobReluType {
                        rnd = rnd,
                        oob = oob,
                        relu = relu,
                        type_ = type_,
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

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::fma::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Abtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Abtype::Bf16),
                map(string_p(".f16"), |_, _span| Abtype::F16)
            )
        }
    }

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Rnd::Rn),
                map(string_p(".rz"), |_, _span| Rnd::Rz),
                map(string_p(".rm"), |_, _span| Rnd::Rm),
                map(string_p(".rp"), |_, _span| Rnd::Rp)
            )
        }
    }

    impl PtxParser for FmaRndSatF32Abtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("fma"),
                    Rnd::parse(),
                    map(optional(string_p(".sat")), |value, _| value.is_some()),
                    string_p(".f32"),
                    Abtype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, rnd, sat, f32, abtype, d, _, a, _, b, _, c, _), span| {
                    ok!(FmaRndSatF32Abtype {
                        rnd = rnd,
                        sat = sat,
                        f32 = f32,
                        abtype = abtype,
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
