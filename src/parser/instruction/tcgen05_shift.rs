//! Original PTX specification:
//!
//! tcgen05.shift.cta_group.down  [taddr];
//! .cta_group = { .cta_group::1, .cta_group::2 }

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
    use crate::r#type::instruction::tcgen05_shift::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Tcgen05ShiftCtaGroupDown {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".shift"),
                    CtaGroup::parse(),
                    string_p(".down"),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, shift, cta_group, down, taddr, _), span| {
                    ok!(Tcgen05ShiftCtaGroupDown {
                        shift = shift,
                        cta_group = cta_group,
                        down = down,
                        taddr = taddr,

                    })
                },
            )
        }
    }
}
