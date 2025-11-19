//! Original PTX specification:
//!
//! getctarank{.space}.type d, a;
//! // Get cta rank from source shared memory address in register a.
//! getctarank.shared::cluster.type d, a;
//! // // Get cta rank from shared memory variable.
//! // getctarank.shared::cluster.type d, var;
//! // // Get cta rank from shared memory variable+offset.
//! // getctarank.shared::cluster.type d, var + imm;
//! // Get cta rank from generic address of shared memory variable in register a.
//! getctarank.type d, a;
//! .space = { .shared::cluster };
//! .type  = { .u32, .u64 };

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
    use crate::r#type::instruction::getctarank::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Space::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64)
            )
        }
    }

    impl PtxParser for GetctarankSpaceType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("getctarank"),
                    optional(Space::parse()),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, space, type_, d, _, a, _), span| {
                    ok!(GetctarankSpaceType {
                        space = space,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for GetctarankSharedClusterType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("getctarank"),
                    string_p(".shared::cluster"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, shared_cluster, type_, d, _, a, _), span| {
                    ok!(GetctarankSharedClusterType {
                        shared_cluster = shared_cluster,
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for GetctarankType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("getctarank"),
                    Type::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, type_, d, _, a, _), span| {
                    ok!(GetctarankType {
                        type_ = type_,
                        d = d,
                        a = a,

                    })
                },
            )
        }
    }
}
