//! Original PTX specification:
//!
//! tensormap.cp_fenceproxy.cp_qualifiers.fence_qualifiers.sync.aligned  [dst], [src], size;
//! .cp_qualifiers    = { .global.shared::cta };
//! .fence_qualifiers = { .to_proxy::from_proxy.release.scope };
//! .to_proxy::from_proxy  = { .tensormap::generic };
//! .scope            = { .cta, .cluster, .gpu , .sys };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tensormap_cp_fenceproxy::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for FenceQualifiers {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ToProxyFromProxyReleaseScope
            {
                let saved_pos = stream.position();
                if stream.expect_string(".to_proxy::from_proxy.release.scope").is_ok() {
                    return Ok(FenceQualifiers::ToProxyFromProxyReleaseScope);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".to_proxy::from_proxy.release.scope"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpQualifiers {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try GlobalSharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global.shared::cta").is_ok() {
                    return Ok(CpQualifiers::GlobalSharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global.shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tensormap")?;
            stream.expect_string(".cp_fenceproxy")?;
            let cp_fenceproxy = ();
            let cp_qualifiers = CpQualifiers::parse(stream)?;
            let fence_qualifiers = FenceQualifiers::parse(stream)?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_string(".aligned")?;
            let aligned = ();
            let dst = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            Ok(TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
                cp_fenceproxy,
                cp_qualifiers,
                fence_qualifiers,
                sync,
                aligned,
                dst,
                src,
                size,
            })
        }
    }


}

