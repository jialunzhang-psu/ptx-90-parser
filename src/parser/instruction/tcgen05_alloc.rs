//! Original PTX specification:
//!
//! tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;
//! tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;
//! tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;
//! .cta_group = { .cta_group::1, .cta_group::2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_alloc::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".alloc")?;
            let alloc = ();
            stream.expect_complete()?;
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let shared_cta = stream.expect_string(".shared::cta").is_ok();
            if !shared_cta {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let dst = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let ncols = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
                alloc,
                cta_group,
                sync,
                aligned,
                shared_cta,
                b32,
                dst,
                ncols,
            })
        }
    }


    impl PtxParser for Tcgen05DeallocCtaGroupSyncAlignedB32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".dealloc")?;
            let dealloc = ();
            stream.expect_complete()?;
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_string(".b32")?;
            let b32 = ();
            stream.expect_complete()?;
            let taddr = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let ncols = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05DeallocCtaGroupSyncAlignedB32 {
                dealloc,
                cta_group,
                sync,
                aligned,
                b32,
                taddr,
                ncols,
            })
        }
    }


    impl PtxParser for Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tcgen05")?;
            stream.expect_string(".relinquish_alloc_permit")?;
            let relinquish_alloc_permit = ();
            stream.expect_complete()?;
            let cta_group = CtaGroup::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
                relinquish_alloc_permit,
                cta_group,
                sync,
                aligned,
            })
        }
    }


}

