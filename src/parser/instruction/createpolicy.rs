//! Original PTX specification:
//!
//! // Range-based policy
//! createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy, [a], primary-size, total-size;
//! // Fraction-based policy
//! createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64
//! cache-policy{, fraction};
//! // Converting the access property from CUDA APIs
//! createpolicy.cvt.L2.b64            cache-policy, access-property;
//! .level::primary_priority =   { .L2::evict_last, .L2::evict_normal,
//! .L2::evict_first, .L2::evict_unchanged };
//! .level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };

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
    use crate::r#type::instruction::createpolicy::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelPrimaryPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L2::evict_unchanged"), |_, _span| {
                    LevelPrimaryPriority::L2EvictUnchanged
                }),
                map(string_p(".L2::evict_normal"), |_, _span| {
                    LevelPrimaryPriority::L2EvictNormal
                }),
                map(string_p(".L2::evict_first"), |_, _span| {
                    LevelPrimaryPriority::L2EvictFirst
                }),
                map(string_p(".L2::evict_last"), |_, _span| {
                    LevelPrimaryPriority::L2EvictLast
                })
            )
        }
    }

    impl PtxParser for LevelSecondaryPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L2::evict_unchanged"), |_, _span| {
                    LevelSecondaryPriority::L2EvictUnchanged
                }),
                map(string_p(".L2::evict_first"), |_, _span| {
                    LevelSecondaryPriority::L2EvictFirst
                })
            )
        }
    }

    impl PtxParser for CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("createpolicy"),
                    string_p(".range"),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    LevelPrimaryPriority::parse(),
                    optional(LevelSecondaryPriority::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    range,
                    global,
                    level_primary_priority,
                    level_secondary_priority,
                    b64,
                    cache_policy,
                    _,
                    a,
                    _,
                    primary_size,
                    _,
                    total_size,
                    _,
                ),
                 span| {
                    ok!(CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
                        range = range,
                        global = global,
                        level_primary_priority = level_primary_priority,
                        level_secondary_priority = level_secondary_priority,
                        b64 = b64,
                        cache_policy = cache_policy,
                        a = a,
                        primary_size = primary_size,
                        total_size = total_size,

                    })
                },
            )
        }
    }

    impl PtxParser for CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("createpolicy"),
                    string_p(".fractional"),
                    LevelPrimaryPriority::parse(),
                    optional(LevelSecondaryPriority::parse()),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(
                    _,
                    fractional,
                    level_primary_priority,
                    level_secondary_priority,
                    b64,
                    cache_policy,
                    fraction,
                    _,
                ),
                 span| {
                    ok!(CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 {
                        fractional = fractional,
                        level_primary_priority = level_primary_priority,
                        level_secondary_priority = level_secondary_priority,
                        b64 = b64,
                        cache_policy = cache_policy,
                        fraction = fraction,

                    })
                },
            )
        }
    }

    impl PtxParser for CreatepolicyCvtL2B64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("createpolicy"),
                    string_p(".cvt"),
                    string_p(".L2"),
                    string_p(".b64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, cvt, l2, b64, cache_policy, _, access_property, _), span| {
                    ok!(CreatepolicyCvtL2B64 {
                        cvt = cvt,
                        l2 = l2,
                        b64 = b64,
                        cache_policy = cache_policy,
                        access_property = access_property,

                    })
                },
            )
        }
    }
}
