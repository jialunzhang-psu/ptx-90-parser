//! Original PTX specification:
//!
//! setmaxnreg.action.sync.aligned.u32 imm-reg-count;
//! .action = { .inc, .dec };

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
    use crate::r#type::instruction::setmaxnreg::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Action {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".inc"), |_, _span| Action::Inc),
                map(string_p(".dec"), |_, _span| Action::Dec)
            )
        }
    }

    impl PtxParser for SetmaxnregActionSyncAlignedU32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("setmaxnreg"),
                    Action::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".u32"),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, action, sync, aligned, u32, imm_reg_count, _), span| {
                    ok!(SetmaxnregActionSyncAlignedU32 {
                        action = action,
                        sync = sync,
                        aligned = aligned,
                        u32 = u32,
                        imm_reg_count = imm_reg_count,

                    })
                },
            )
        }
    }
}
