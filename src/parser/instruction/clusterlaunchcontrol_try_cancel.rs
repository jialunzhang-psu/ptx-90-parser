//! Original PTX specification:
//!
//! clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .space = { .shared::cta };

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
    use crate::r#type::instruction::clusterlaunchcontrol_try_cancel::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cta"), |_, _span| Space::SharedCta))
        }
    }

    impl PtxParser
        for ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128
    {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("clusterlaunchcontrol"),
                    string_p(".try_cancel"),
                    string_p(".async"),
                    optional(Space::parse()),
                    CompletionMechanism::parse(),
                    map(
                        optional(string_p(".multicast::cluster::all")),
                        |value, _| value.is_some()
                    ),
                    string_p(".b128"),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    try_cancel,
                    async_,
                    space,
                    completion_mechanism,
                    multicast_cluster_all,
                    b128,
                    addr,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 {
                        try_cancel = try_cancel,
                        async_ = async_,
                        space = space,
                        completion_mechanism = completion_mechanism,
                        multicast_cluster_all = multicast_cluster_all,
                        b128 = b128,
                        addr = addr,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}
