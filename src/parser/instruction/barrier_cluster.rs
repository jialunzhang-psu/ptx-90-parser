//! Original PTX specification:
//!
//! barrier.cluster.arrive{.sem}{.aligned};
//! barrier.cluster.wait{.acquire}{.aligned};
//! .sem = {.release, .relaxed};

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
    use crate::r#type::instruction::barrier_cluster::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".release"), |_, _span| Sem::Release),
                map(string_p(".relaxed"), |_, _span| Sem::Relaxed)
            )
        }
    }

    impl PtxParser for BarrierClusterArriveSemAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    string_p(".cluster"),
                    string_p(".arrive"),
                    optional(Sem::parse()),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    semicolon_p()
                ),
                |(_, cluster, arrive, sem, aligned, _), span| {
                    ok!(BarrierClusterArriveSemAligned {
                        cluster = cluster,
                        arrive = arrive,
                        sem = sem,
                        aligned = aligned,

                    })
                },
            )
        }
    }

    impl PtxParser for BarrierClusterWaitAcquireAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("barrier"),
                    string_p(".cluster"),
                    string_p(".wait"),
                    map(optional(string_p(".acquire")), |value, _| value.is_some()),
                    map(optional(string_p(".aligned")), |value, _| value.is_some()),
                    semicolon_p()
                ),
                |(_, cluster, wait, acquire, aligned, _), span| {
                    ok!(BarrierClusterWaitAcquireAligned {
                        cluster = cluster,
                        wait = wait,
                        acquire = acquire,
                        aligned = aligned,

                    })
                },
            )
        }
    }
}
