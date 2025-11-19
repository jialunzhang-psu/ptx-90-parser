//! Original PTX specification:
//!
//! tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;
//! tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;
//! tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;
//! .cta_group = { .cta_group::1, .cta_group::2 };

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
    use crate::r#type::instruction::tcgen05_alloc::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cta_group::1"), |_, _span| CtaGroup::CtaGroup1),
                map(string_p(".cta_group::2"), |_, _span| CtaGroup::CtaGroup2)
            )
        }
    }

    impl PtxParser for Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".alloc"),
                    CtaGroup::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    map(optional(string_p(".shared::cta")), |value, _| value
                        .is_some()),
                    string_p(".b32"),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, alloc, cta_group, sync, aligned, shared_cta, b32, dst, _, ncols, _), span| {
                    ok!(Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
                        alloc = alloc,
                        cta_group = cta_group,
                        sync = sync,
                        aligned = aligned,
                        shared_cta = shared_cta,
                        b32 = b32,
                        dst = dst,
                        ncols = ncols,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05DeallocCtaGroupSyncAlignedB32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".dealloc"),
                    CtaGroup::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".b32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, dealloc, cta_group, sync, aligned, b32, taddr, _, ncols, _), span| {
                    ok!(Tcgen05DeallocCtaGroupSyncAlignedB32 {
                        dealloc = dealloc,
                        cta_group = cta_group,
                        sync = sync,
                        aligned = aligned,
                        b32 = b32,
                        taddr = taddr,
                        ncols = ncols,

                    })
                },
            )
        }
    }

    impl PtxParser for Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tcgen05"),
                    string_p(".relinquish_alloc_permit"),
                    CtaGroup::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    semicolon_p()
                ),
                |(_, relinquish_alloc_permit, cta_group, sync, aligned, _), span| {
                    ok!(Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
                        relinquish_alloc_permit = relinquish_alloc_permit,
                        cta_group = cta_group,
                        sync = sync,
                        aligned = aligned,

                    })
                },
            )
        }
    }
}
