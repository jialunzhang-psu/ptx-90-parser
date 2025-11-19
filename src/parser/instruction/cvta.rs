//! Original PTX specification:
//!
//! // convert const, global, local, or shared address to generic address
//! cvta.space.size  p, a;        // source address in register a
//! // cvta.space.size  p, var;      // get generic address of var
//! // cvta.space.size  p, var+imm;  // generic address of var+offset
//! // convert generic address to const, global, local, or shared address
//! cvta.to.space.size  p, a;
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };
//! .size  = { .u32, .u64 };

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
    use crate::r#type::instruction::cvta::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Size {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Size::U32),
                map(string_p(".u64"), |_, _span| Size::U64)
            )
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cluster"), |_, _span| {
                    Space::SharedCluster
                }),
                map(string_p(".param::entry"), |_, _span| Space::ParamEntry),
                map(string_p(".shared::cta"), |_, _span| Space::SharedCta),
                map(string_p(".global"), |_, _span| Space::Global),
                map(string_p(".shared"), |_, _span| Space::Shared),
                map(string_p(".const"), |_, _span| Space::Const),
                map(string_p(".local"), |_, _span| Space::Local),
                map(string_p(".param"), |_, _span| Space::Param)
            )
        }
    }

    impl PtxParser for CvtaSpaceSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvta"),
                    Space::parse(),
                    Size::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, space, size, p, _, a, _), span| {
                    ok!(CvtaSpaceSize {
                        space = space,
                        size = size,
                        p = p,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for CvtaToSpaceSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cvta"),
                    string_p(".to"),
                    Space::parse(),
                    Size::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, to, space, size, p, _, a, _), span| {
                    ok!(CvtaToSpaceSize {
                        to = to,
                        space = space,
                        size = size,
                        p = p,
                        a = a,

                    })
                },
            )
        }
    }
}
