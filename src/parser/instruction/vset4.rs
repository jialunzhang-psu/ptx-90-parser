//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vset4.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vset4.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .mask  = { .b0,
//! .b1, .b10
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! defaults to .b3210
//! .asel = .bsel = { .b00, .b01, .b02, .b03, .b04, .b05, .b06, .b07,
//!                   .b10, .b11, .b12, .b13, .b14, .b15, .b16, .b17,
//!                   .b20, .b21, .b22, .b23, .b24, .b25, .b26, .b27,
//!                   .b30, .b31, .b32, .b33, .b34, .b35, .b36, .b37,
//!                   .b40, .b41, .b42, .b43, .b44, .b45, .b46, .b47,
//!                   .b50, .b51, .b52, .b53, .b54, .b55, .b56, .b57,
//!                   .b60, .b61, .b62, .b63, .b64, .b65, .b66, .b67,
//!                   .b70, .b71, .b72, .b73, .b74, .b75, .b76, .b77
//!                   } //.bxyzw, where x,y,z,w are from { 0, ..., 7 };
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

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
    use crate::r#type::instruction::vset4::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
                map(string_p(".b00"), |_, _span| Bsel::B00),
                map(string_p(".b01"), |_, _span| Bsel::B01),
                map(string_p(".b02"), |_, _span| Bsel::B02),
                map(string_p(".b03"), |_, _span| Bsel::B03),
                map(string_p(".b04"), |_, _span| Bsel::B04),
                map(string_p(".b05"), |_, _span| Bsel::B05),
                map(string_p(".b06"), |_, _span| Bsel::B06),
                map(string_p(".b07"), |_, _span| Bsel::B07),
                map(string_p(".b10"), |_, _span| Bsel::B10),
                map(string_p(".b11"), |_, _span| Bsel::B11),
                map(string_p(".b12"), |_, _span| Bsel::B12),
                map(string_p(".b13"), |_, _span| Bsel::B13),
                map(string_p(".b14"), |_, _span| Bsel::B14),
                map(string_p(".b15"), |_, _span| Bsel::B15),
                map(string_p(".b16"), |_, _span| Bsel::B16),
                map(string_p(".b17"), |_, _span| Bsel::B17),
                map(string_p(".b20"), |_, _span| Bsel::B20),
                map(string_p(".b21"), |_, _span| Bsel::B21),
                map(string_p(".b22"), |_, _span| Bsel::B22),
                map(string_p(".b23"), |_, _span| Bsel::B23),
                map(string_p(".b24"), |_, _span| Bsel::B24),
                map(string_p(".b25"), |_, _span| Bsel::B25),
                map(string_p(".b26"), |_, _span| Bsel::B26),
                map(string_p(".b27"), |_, _span| Bsel::B27),
                map(string_p(".b30"), |_, _span| Bsel::B30),
                map(string_p(".b31"), |_, _span| Bsel::B31),
                map(string_p(".b32"), |_, _span| Bsel::B32),
                map(string_p(".b33"), |_, _span| Bsel::B33),
                map(string_p(".b34"), |_, _span| Bsel::B34),
                map(string_p(".b35"), |_, _span| Bsel::B35),
                map(string_p(".b36"), |_, _span| Bsel::B36),
                map(string_p(".b37"), |_, _span| Bsel::B37),
                map(string_p(".b40"), |_, _span| Bsel::B40),
                map(string_p(".b41"), |_, _span| Bsel::B41),
                map(string_p(".b42"), |_, _span| Bsel::B42),
                map(string_p(".b43"), |_, _span| Bsel::B43),
                map(string_p(".b44"), |_, _span| Bsel::B44),
                map(string_p(".b45"), |_, _span| Bsel::B45),
                map(string_p(".b46"), |_, _span| Bsel::B46),
                map(string_p(".b47"), |_, _span| Bsel::B47),
                map(string_p(".b50"), |_, _span| Bsel::B50),
                map(string_p(".b51"), |_, _span| Bsel::B51),
                map(string_p(".b52"), |_, _span| Bsel::B52),
                map(string_p(".b53"), |_, _span| Bsel::B53),
                map(string_p(".b54"), |_, _span| Bsel::B54),
                map(string_p(".b55"), |_, _span| Bsel::B55),
                map(string_p(".b56"), |_, _span| Bsel::B56),
                map(string_p(".b57"), |_, _span| Bsel::B57),
                map(string_p(".b60"), |_, _span| Bsel::B60),
                map(string_p(".b61"), |_, _span| Bsel::B61),
                map(string_p(".b62"), |_, _span| Bsel::B62),
                map(string_p(".b63"), |_, _span| Bsel::B63),
                map(string_p(".b64"), |_, _span| Bsel::B64),
                map(string_p(".b65"), |_, _span| Bsel::B65),
                map(string_p(".b66"), |_, _span| Bsel::B66),
                map(string_p(".b67"), |_, _span| Bsel::B67),
                map(string_p(".b70"), |_, _span| Bsel::B70),
                map(string_p(".b71"), |_, _span| Bsel::B71),
                map(string_p(".b72"), |_, _span| Bsel::B72),
                map(string_p(".b73"), |_, _span| Bsel::B73),
                map(string_p(".b74"), |_, _span| Bsel::B74),
                map(string_p(".b75"), |_, _span| Bsel::B75),
                map(string_p(".b76"), |_, _span| Bsel::B76),
                map(string_p(".b77"), |_, _span| Bsel::B77)
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

    impl PtxParser for Cmp {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".eq"), |_, _span| Cmp::Eq),
                map(string_p(".ne"), |_, _span| Cmp::Ne),
                map(string_p(".lt"), |_, _span| Cmp::Lt),
                map(string_p(".le"), |_, _span| Cmp::Le),
                map(string_p(".gt"), |_, _span| Cmp::Gt),
                map(string_p(".ge"), |_, _span| Cmp::Ge)
            )
        }
    }

    impl PtxParser for Mask {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b10.b2"), |_, _span| Mask::B10B2),
                map(string_p(".b3210"), |_, _span| Mask::B3210),
                map(string_p(".b210"), |_, _span| Mask::B210),
                map(string_p(".b310"), |_, _span| Mask::B310),
                map(string_p(".b320"), |_, _span| Mask::B320),
                map(string_p(".b321"), |_, _span| Mask::B321),
                map(string_p(".b20"), |_, _span| Mask::B20),
                map(string_p(".b21"), |_, _span| Mask::B21),
                map(string_p(".b30"), |_, _span| Mask::B30),
                map(string_p(".b31"), |_, _span| Mask::B31),
                map(string_p(".b32"), |_, _span| Mask::B32),
                map(string_p(".b0"), |_, _span| Mask::B0),
                map(string_p(".b1"), |_, _span| Mask::B1),
                map(string_p(".b3"), |_, _span| Mask::B3)
            )
        }
    }

    impl PtxParser for Vset4AtypeBtypeCmp {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset4"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(optional(string_p(".asel")), |value, _| value.is_some()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, atype, btype, cmp, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vset4AtypeBtypeCmp {
                        atype = atype,
                        btype = btype,
                        cmp = cmp,
                        d = d,
                        mask = mask,
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

    impl PtxParser for Vset4AtypeBtypeCmpAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset4"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(optional(string_p(".asel")), |value, _| value.is_some()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, atype, btype, cmp, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vset4AtypeBtypeCmpAdd {
                        atype = atype,
                        btype = btype,
                        cmp = cmp,
                        add = add,
                        d = d,
                        mask = mask,
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
