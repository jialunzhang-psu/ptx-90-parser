//! Original PTX specification:
//!
//! mbarrier.expect_tx{.sem}{.scope}{.space}.b64 [addr], txCount;
//! .sem   = { .relaxed };
//! .scope = { .cta, .cluster };
//! .space = { .shared, .shared::cta, .shared::cluster };

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
    use crate::r#type::instruction::mbarrier_expect_tx::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".cta"), |_, _span| Scope::Cta)
            )
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".relaxed"), |_, _span| Sem::Relaxed))
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cluster"), |_, _span| {
                    Space::SharedCluster
                }),
                map(string_p(".shared::cta"), |_, _span| Space::SharedCta),
                map(string_p(".shared"), |_, _span| Space::Shared)
            )
        }
    }

    impl PtxParser for MbarrierExpectTxSemScopeSpaceB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".expect_tx"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Space::parse()),
                    string_p(".b64"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, expect_tx, sem, scope, space, b64, addr, _, txcount, _), span| {
                    ok!(MbarrierExpectTxSemScopeSpaceB64 {
                        expect_tx = expect_tx,
                        sem = sem,
                        scope = scope,
                        space = space,
                        b64 = b64,
                        addr = addr,
                        txcount = txcount,

                    })
                },
            )
        }
    }
}
