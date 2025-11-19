//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vset2.atype.btype.cmp  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vset2.atype.btype.cmp.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; // { .hxy, where x,y are from { 0, 1, 2, 3 } };
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

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
    use crate::r#type::instruction::vset2::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Asel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".h00"), |_, _span| Asel::H00),
                map(string_p(".h01"), |_, _span| Asel::H01),
                map(string_p(".h02"), |_, _span| Asel::H02),
                map(string_p(".h03"), |_, _span| Asel::H03),
                map(string_p(".h10"), |_, _span| Asel::H10),
                map(string_p(".h11"), |_, _span| Asel::H11),
                map(string_p(".h12"), |_, _span| Asel::H12),
                map(string_p(".h13"), |_, _span| Asel::H13),
                map(string_p(".h20"), |_, _span| Asel::H20),
                map(string_p(".h21"), |_, _span| Asel::H21),
                map(string_p(".h22"), |_, _span| Asel::H22),
                map(string_p(".h23"), |_, _span| Asel::H23),
                map(string_p(".h30"), |_, _span| Asel::H30),
                map(string_p(".h31"), |_, _span| Asel::H31),
                map(string_p(".h32"), |_, _span| Asel::H32),
                map(string_p(".h33"), |_, _span| Asel::H33)
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
                map(string_p(".h00"), |_, _span| Bsel::H00),
                map(string_p(".h01"), |_, _span| Bsel::H01),
                map(string_p(".h02"), |_, _span| Bsel::H02),
                map(string_p(".h03"), |_, _span| Bsel::H03),
                map(string_p(".h10"), |_, _span| Bsel::H10),
                map(string_p(".h11"), |_, _span| Bsel::H11),
                map(string_p(".h12"), |_, _span| Bsel::H12),
                map(string_p(".h13"), |_, _span| Bsel::H13),
                map(string_p(".h20"), |_, _span| Bsel::H20),
                map(string_p(".h21"), |_, _span| Bsel::H21),
                map(string_p(".h22"), |_, _span| Bsel::H22),
                map(string_p(".h23"), |_, _span| Bsel::H23),
                map(string_p(".h30"), |_, _span| Bsel::H30),
                map(string_p(".h31"), |_, _span| Bsel::H31),
                map(string_p(".h32"), |_, _span| Bsel::H32),
                map(string_p(".h33"), |_, _span| Bsel::H33)
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
                map(string_p(".h10"), |_, _span| Mask::H10),
                map(string_p(".h0"), |_, _span| Mask::H0),
                map(string_p(".h1"), |_, _span| Mask::H1)
            )
        }
    }

    impl PtxParser for Vset2AtypeBtypeCmp {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset2"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
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
                |(_, atype, btype, cmp, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vset2AtypeBtypeCmp {
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

    impl PtxParser for Vset2AtypeBtypeCmpAdd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset2"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
                    string_p(".add"),
                    GeneralOperand::parse(),
                    optional(Mask::parse()),
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
                |(_, atype, btype, cmp, add, d, mask, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(Vset2AtypeBtypeCmpAdd {
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
