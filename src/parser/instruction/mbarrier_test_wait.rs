//! Original PTX specification:
//!
//! mbarrier.test_wait{.sem}{.scope}{.state}.b64        waitComplete, [addr], state;
//! mbarrier.test_wait.parity{.sem}{.scope}{.state}.b64 waitComplete, [addr], phaseParity;
//! mbarrier.try_wait{.sem}{.scope}{.state}.b64         waitComplete, [addr], state {, suspendTimeHint};
//! mbarrier.try_wait.parity{.sem}{.scope}{.state}.b64  waitComplete, [addr], phaseParity {, suspendTimeHint};
//! .sem   = { .acquire, .relaxed };
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
    use crate::r#type::instruction::mbarrier_test_wait::section_0::*;

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
                map(string_p(".acquire"), |_, _span| Sem::Acquire),
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

    impl PtxParser for MbarrierTestWaitSemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".test_wait"),
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
                |(_, test_wait, sem, scope, state, b64, waitcomplete, _, addr, _, state2, _),
                 span| {
                    ok!(MbarrierTestWaitSemScopeStateB64 {
                        test_wait = test_wait,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        waitcomplete = waitcomplete,
                        addr = addr,
                        state2 = state2,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierTestWaitParitySemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".test_wait"),
                    string_p(".parity"),
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
                |(
                    _,
                    test_wait,
                    parity,
                    sem,
                    scope,
                    state,
                    b64,
                    waitcomplete,
                    _,
                    addr,
                    _,
                    phaseparity,
                    _,
                ),
                 span| {
                    ok!(MbarrierTestWaitParitySemScopeStateB64 {
                        test_wait = test_wait,
                        parity = parity,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        waitcomplete = waitcomplete,
                        addr = addr,
                        phaseparity = phaseparity,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierTryWaitSemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".try_wait"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(State::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    try_wait,
                    sem,
                    scope,
                    state,
                    b64,
                    waitcomplete,
                    _,
                    addr,
                    _,
                    state2,
                    suspendtimehint,
                    _,
                ),
                 span| {
                    ok!(MbarrierTryWaitSemScopeStateB64 {
                        try_wait = try_wait,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        waitcomplete = waitcomplete,
                        addr = addr,
                        state2 = state2,
                        suspendtimehint = suspendtimehint,

                    })
                },
            )
        }
    }

    impl PtxParser for MbarrierTryWaitParitySemScopeStateB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mbarrier"),
                    string_p(".try_wait"),
                    string_p(".parity"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(State::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    try_wait,
                    parity,
                    sem,
                    scope,
                    state,
                    b64,
                    waitcomplete,
                    _,
                    addr,
                    _,
                    phaseparity,
                    suspendtimehint,
                    _,
                ),
                 span| {
                    ok!(MbarrierTryWaitParitySemScopeStateB64 {
                        try_wait = try_wait,
                        parity = parity,
                        sem = sem,
                        scope = scope,
                        state = state,
                        b64 = b64,
                        waitcomplete = waitcomplete,
                        addr = addr,
                        phaseparity = phaseparity,
                        suspendtimehint = suspendtimehint,

                    })
                },
            )
        }
    }
}
