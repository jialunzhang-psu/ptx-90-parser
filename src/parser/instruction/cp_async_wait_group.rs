//! Original PTX specification:
//!
//! cp.async.wait_group N;
//! cp.async.wait_all ;

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
    use crate::r#type::instruction::cp_async_wait_group::section_0::*;

    impl PtxParser for CpAsyncWaitGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".wait_group"),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, async_, wait_group, n, _), span| {
                    ok!(CpAsyncWaitGroup {
                        async_ = async_,
                        wait_group = wait_group,
                        n = n,

                    })
                },
            )
        }
    }

    impl PtxParser for CpAsyncWaitAll {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".wait_all"),
                    semicolon_p()
                ),
                |(_, async_, wait_all, _), span| {
                    ok!(CpAsyncWaitAll {
                        async_ = async_,
                        wait_all = wait_all,

                    })
                },
            )
        }
    }
}
