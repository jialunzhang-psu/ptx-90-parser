//! Original PTX specification:
//!
//! prefetch{.space}.level                    [a];   // prefetch to data cache
//! prefetch.global.level::eviction_priority  [a];   // prefetch to data cache
//! prefetchu.L1  [a];             // prefetch to uniform cache
//! prefetch{.tensormap_space}.tensormap [a];  // prefetch the tensormap
//! .space =                    { .global, .local };
//! .level =                    { .L1, .L2 };
//! .level::eviction_priority = { .L2::evict_last, .L2::evict_normal };
//! .tensormap_space =          { .const, .param };

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
    use crate::r#type::instruction::prefetch::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Level {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L1"), |_, _span| Level::L1),
                map(string_p(".L2"), |_, _span| Level::L2)
            )
        }
    }

    impl PtxParser for LevelEvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".L2::evict_normal"), |_, _span| {
                    LevelEvictionPriority::L2EvictNormal
                }),
                map(string_p(".L2::evict_last"), |_, _span| {
                    LevelEvictionPriority::L2EvictLast
                })
            )
        }
    }

    impl PtxParser for Space {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".global"), |_, _span| Space::Global),
                map(string_p(".local"), |_, _span| Space::Local)
            )
        }
    }

    impl PtxParser for TensormapSpace {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".const"), |_, _span| TensormapSpace::Const),
                map(string_p(".param"), |_, _span| TensormapSpace::Param)
            )
        }
    }

    impl PtxParser for PrefetchSpaceLevel {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("prefetch"),
                    optional(Space::parse()),
                    Level::parse(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, space, level, a, _), span| {
                    ok!(PrefetchSpaceLevel {
                        space = space,
                        level = level,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for PrefetchGlobalLevelEvictionPriority {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("prefetch"),
                    string_p(".global"),
                    LevelEvictionPriority::parse(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, global, level_eviction_priority, a, _), span| {
                    ok!(PrefetchGlobalLevelEvictionPriority {
                        global = global,
                        level_eviction_priority = level_eviction_priority,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for PrefetchuL1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("prefetchu"),
                    string_p(".L1"),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, l1, a, _), span| {
                    ok!(PrefetchuL1 {
                        l1 = l1,
                        a = a,

                    })
                },
            )
        }
    }

    impl PtxParser for PrefetchTensormapSpaceTensormap {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("prefetch"),
                    optional(TensormapSpace::parse()),
                    string_p(".tensormap"),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(_, tensormap_space, tensormap, a, _), span| {
                    ok!(PrefetchTensormapSpaceTensormap {
                        tensormap_space = tensormap_space,
                        tensormap = tensormap,
                        a = a,

                    })
                },
            )
        }
    }
}
