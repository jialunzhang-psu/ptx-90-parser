//! Original PTX specification:
//!
//! mbarrier.arrive{.sem}{.scope}{.state}.b64           state, [addr]{, count};
//! mbarrier.arrive{.sem}{.scope}{.shared::cluster}.b64         _, [addr] {,count};
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.state}.b64 state, [addr], txCount;
//! mbarrier.arrive.expect_tx{.sem}{.scope}{.shared::cluster}.b64   _, [addr], txCount;
//! mbarrier.arrive.noComplete{.release}{.cta}{.state}.b64  state, [addr], count;
//! .sem   = { .release, .relaxed };
//! .scope = { .cta, .cluster };
//! .state = { .shared, .shared::cta}

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
    use crate::r#type::instruction::mbarrier_arrive::section_0::*;

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
            alt!(
                map(string_p(".release"), |_, _span| Sem::Release),
                map(string_p(".relaxed"), |_, _span| Sem::Relaxed)
            )
        }
    }

    impl PtxParser for State {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| State::SharedCta),
                map(string_p(".shared"), |_, _span| State::Shared)
            )
        }
    }

    impl PtxParser for MbarrierArriveSemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".arrive"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(State::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, arrive, sem, scope, state, b64, state2, _, addr, count, _), span| {
                    ok!(MbarrierArriveSemScopeStateB64 {
                        arrive = arrive,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        state2 = state2,
                        addr = addr,
                        count = count,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierArriveSemScopeSharedClusterB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".arrive"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    map(optional(string_p(".shared::cluster")), |value, _| value
                        .is_some()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, arrive, sem, scope, shared_cluster, b64, operand, _, addr, count, _), span| {
                    ok!(MbarrierArriveSemScopeSharedClusterB64 {
                        arrive = arrive,
                        sem = sem,
                        scope = scope,
                        shared_cluster = shared_cluster,
                        b64 = b64,
                        operand = operand,
                        addr = addr,
                        count = count,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierArriveExpectTxSemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".arrive"),
                    string_p(".expect_tx"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(State::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, arrive, expect_tx, sem, scope, state, b64, state2, _, addr, _, txcount, _),
                 span| {
                    ok!(MbarrierArriveExpectTxSemScopeStateB64 {
                        arrive = arrive,
                        expect_tx = expect_tx,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        state2 = state2,
                        addr = addr,
                        txcount = txcount,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierArriveExpectTxSemScopeSharedClusterB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".arrive"),
                    string_p(".expect_tx"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    map(optional(string_p(".shared::cluster")), |value, _| value
                        .is_some()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    arrive,
                    expect_tx,
                    sem,
                    scope,
                    shared_cluster,
                    b64,
                    operand,
                    _,
                    addr,
                    _,
                    txcount,
                    _,
                ),
                 span| {
                    ok!(MbarrierArriveExpectTxSemScopeSharedClusterB64 {
                        arrive = arrive,
                        expect_tx = expect_tx,
                        sem = sem,
                        scope = scope,
                        shared_cluster = shared_cluster,
                        b64 = b64,
                        operand = operand,
                        addr = addr,
                        txcount = txcount,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierArriveNocompleteReleaseCtaStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".arrive"),
                    string_p(".noComplete"),
                    map(optional(string_p(".release")), |value, _| value.is_some()),
                    map(optional(string_p(".cta")), |value, _| value.is_some()),
                    optional(State::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    arrive,
                    nocomplete,
                    release,
                    cta,
                    state,
                    b64,
                    state2,
                    _,
                    addr,
                    _,
                    count,
                    _,
                ),
                 span| {
                    ok!(MbarrierArriveNocompleteReleaseCtaStateB64 {
                        arrive = arrive,
                        nocomplete = nocomplete,
                        release = release,
                        cta = cta,
                        state = state,
                        b64 = b64,
                        state2 = state2,
                        addr = addr,
                        count = count,

                    })
                },
            )
        }
    }
}
