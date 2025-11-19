//! Original PTX specification:
//!
//! cp.async.commit_group ;

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
    use crate::r#type::instruction::cp_async_commit_group::section_0::*;

    impl PtxParser for CpAsyncCommitGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("cp"),
                    string_p(".async"),
                    string_p(".commit_group"),
                    semicolon_p()
                ),
                |(_, async_, commit_group, _), span| {
                    ok!(CpAsyncCommitGroup {
                        async_ = async_,
                        commit_group = commit_group,

                    })
                },
            )
        }
    }
}
