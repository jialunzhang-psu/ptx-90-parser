//! Original PTX specification:
//!
//! tcgen05.wait_operation.sync.aligned;
//! .wait_operation = { .wait::ld, .wait::st }

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
    use crate::r#type::instruction::tcgen05_wait::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for WaitOperation {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".wait::ld"), |_, _span| WaitOperation::WaitLd),
                map(string_p(".wait::st"), |_, _span| WaitOperation::WaitSt)
            )
        }
    }

    impl PtxParser for Tcgen05WaitOperationSyncAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    WaitOperation::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    semicolon_p()
                ),
                |(_, wait_operation, sync, aligned, _), span| {
                    ok!(Tcgen05WaitOperationSyncAligned {
                        wait_operation = wait_operation,
                        sync = sync,
                        aligned = aligned,

                    })
                },
            )
        }
    }
}
