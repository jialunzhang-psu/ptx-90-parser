//! Original PTX specification:
//!
//! // 32-bit scalar operation, with optional secondary operation
//! vset.atype.btype.cmp       d, a{.asel}, b{.bsel};
//! vset.atype.btype.cmp.op2   d, a{.asel}, b{.bsel}, c;
//! // 32-bit scalar operation, with optional data merge
//! vset.atype.btype.cmp  d.dsel, a{.asel}, b{.bsel}, c;
//! .atype = .btype = { .u32, .s32 };
//! .cmp   = { .eq, .ne, .lt, .le, .gt, .ge };
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
    use crate::r#type::instruction::vset::section_0::*;

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

    impl PtxParser for Op2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".add"), |_, _span| Op2::Add),
                map(string_p(".min"), |_, _span| Op2::Min),
                map(string_p(".max"), |_, _span| Op2::Max)
            )
        }
    }

    impl PtxParser for VsetAtypeBtypeCmp {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Asel::parse()),
                    comma_p(),
                    GeneralOperand::parse(),
                    optional(Bsel::parse()),
                    semicolon_p()
                ),
                |(_, atype, btype, cmp, d, _, a, asel, _, b, bsel, _), span| {
                    ok!(VsetAtypeBtypeCmp {
                        atype = atype,
                        btype = btype,
                        cmp = cmp,
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

    impl PtxParser for VsetAtypeBtypeCmpOp2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
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
                |(_, atype, btype, cmp, op2, d, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VsetAtypeBtypeCmpOp2 {
                        atype = atype,
                        btype = btype,
                        cmp = cmp,
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

    impl PtxParser for VsetAtypeBtypeCmp1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("vset"),
                    Atype::parse(),
                    Btype::parse(),
                    Cmp::parse(),
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
                |(_, atype, btype, cmp, d, dsel, _, a, asel, _, b, bsel, _, c, _), span| {
                    ok!(VsetAtypeBtypeCmp1 {
                        atype = atype,
                        btype = btype,
                        cmp = cmp,
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
