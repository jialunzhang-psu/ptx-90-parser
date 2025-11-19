//! Original PTX specification:
//!
//! isspacep.space  p, a;    // result is .pred
//! .space = { .const, .global, .local, .shared, .shared::cta, .shared::cluster, .param, .param::entry };

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
    use crate::r#type::instruction::isspacep::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for IsspacepSpace {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("isspacep"),
                    Space::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, space, p, _, a, _), span| {
                    ok!(IsspacepSpace {
                        space = space,
                        p = p,
                        a = a,

                    })
                },
            )
        }
    }
}
