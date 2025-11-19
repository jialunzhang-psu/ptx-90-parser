//! Original PTX specification:
//!
//! bar.warp.sync      membermask;

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
    use crate::r#type::instruction::bar_warp_sync::section_0::*;

    impl PtxParser for BarWarpSync {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("bar"),
                    string_p(".warp"),
                    string_p(".sync"),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, warp, sync, membermask, _), span| {
                    ok!(BarWarpSync {
                        warp = warp,
                        sync = sync,
                        membermask = membermask,

                    })
                },
            )
        }
    }
}
