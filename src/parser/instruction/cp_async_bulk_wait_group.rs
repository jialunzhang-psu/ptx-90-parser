//! Original PTX specification:
//!
//! cp.async.bulk.wait_group{.read} N;

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
    use crate::r#type::instruction::cp_async_bulk_wait_group::section_0::*;

    impl PtxParser for CpAsyncBulkWaitGroupRead {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".bulk"),
                    string_p(".wait_group"),
                    map(optional(string_p(".read")), |value, _| value.is_some()),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, async_, bulk, wait_group, read, n, _), span| {
                    ok!(CpAsyncBulkWaitGroupRead {
                        async_ = async_,
                        bulk = bulk,
                        wait_group = wait_group,
                        read = read,
                        n = n,

                    })
                },
            )
        }
    }
}
