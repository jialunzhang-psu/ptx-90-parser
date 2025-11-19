//! Original PTX specification:
//!
//! tcgen05.commit.cta_group.completion_mechanism{.shared::cluster}{.multicast}.b64
//! [mbar] {, ctaMask};
//! .completion_mechanism = { .mbarrier::arrive::one };
//! .cta_group            = { .cta_group::1, .cta_group::2 };
//! .multicast            = { .multicast::cluster };

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
    use crate::r#type::instruction::tcgen05_commit::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".mbarrier::arrive::one"), |_, _span| {
                CompletionMechanism::MbarrierArriveOne
            }))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Multicast {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".multicast::cluster"), |_, _span| {
                Multicast::MulticastCluster
            }))
        }
    }

    impl PtxParser for Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".commit"),
                    CtaGroup::parse(),
                    CompletionMechanism::parse(),
                    map(optional(string_p(".shared::cluster")), |value, _| value
                        .is_some()),
                    optional(Multicast::parse()),
                    string_p(".b64"),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    commit,
                    cta_group,
                    completion_mechanism,
                    shared_cluster,
                    multicast,
                    b64,
                    mbar,
                    ctamask,
                    _,
                ),
                 span| {
                    ok!(Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 {
                        commit = commit,
                        cta_group = cta_group,
                        completion_mechanism = completion_mechanism,
                        shared_cluster = shared_cluster,
                        multicast = multicast,
                        b64 = b64,
                        mbar = mbar,
                        ctamask = ctamask,

                    })
                },
            )
        }
    }
}
