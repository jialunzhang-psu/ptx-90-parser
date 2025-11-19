//! Original PTX specification:
//!
//! applypriority{.global}.level::eviction_priority  [a], size;
//! .level::eviction_priority = { .L2::evict_normal };

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
    use crate::r#type::instruction::applypriority::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelEvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".L2::evict_normal"), |_, _span| {
                LevelEvictionPriority::L2EvictNormal
            }))
        }
    }

    impl PtxParser for ApplypriorityGlobalLevelEvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("applypriority"),
                    map(optional(string_p(".global")), |value, _| value.is_some()),
                    LevelEvictionPriority::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, global, level_eviction_priority, a, _, size, _), span| {
                    ok!(ApplypriorityGlobalLevelEvictionPriority {
                        global = global,
                        level_eviction_priority = level_eviction_priority,
                        a = a,
                        size = size,

                    })
                },
            )
        }
    }
}
