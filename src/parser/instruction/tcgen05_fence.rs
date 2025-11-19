//! Original PTX specification:
//!
//! tcgen05.fence::before_thread_sync ;
//! tcgen05.fence::after_thread_sync  ;

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
    use crate::r#type::instruction::tcgen05_fence::section_0::*;

    impl PtxParser for Tcgen05FenceBeforeThreadSync {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".fence::before_thread_sync"),
                    semicolon_p()
                ),
                |(_, fence_before_thread_sync, _), span| {
                    ok!(Tcgen05FenceBeforeThreadSync {
                        fence_before_thread_sync = fence_before_thread_sync,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05FenceAfterThreadSync {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".fence::after_thread_sync"),
                    semicolon_p()
                ),
                |(_, fence_after_thread_sync, _), span| {
                    ok!(Tcgen05FenceAfterThreadSync {
                        fence_after_thread_sync = fence_after_thread_sync,

                    })
                },
            )
        }
    }
}
