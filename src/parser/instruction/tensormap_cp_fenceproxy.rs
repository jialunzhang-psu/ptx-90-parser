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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global.shared::cta"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for FenceQualifiers {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try ToProxyFromProxyReleaseScope
            {
                let saved_seq_pos = stream.position();
                match (|| -> Result<_, PtxParseError> {
                    let to_proxy_from_proxy = ToProxyFromProxy::parse(stream)?;
                    stream.expect_string(".release")?;
                    let release = ();
                    let scope = Scope::parse(stream)?;
                    Ok((to_proxy_from_proxy, release, scope))
                })() {
                    Ok((to_proxy_from_proxy, release, scope)) => {
                        return Ok(FenceQualifiers::ToProxyFromProxyReleaseScope(
                            to_proxy_from_proxy,
                            release,
                            scope,
                        ));
                    }
                    Err(_) => {
                        stream.set_position(saved_seq_pos);
                    }
                }
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &["<complex>"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for ToProxyFromProxy {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try TensormapGeneric
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tensormap::generic").is_ok() {
                    return Ok(ToProxyFromProxy::TensormapGeneric);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tensormap::generic"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("tensormap")?;
            stream.expect_string(".cp_fenceproxy")?;
            let cp_fenceproxy = ();
            stream.expect_complete()?;
            let cp_qualifiers = CpQualifiers::parse(stream)?;
            stream.expect_complete()?;
            let fence_qualifiers = FenceQualifiers::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".sync")?;
            let sync = ();
            stream.expect_complete()?;
            stream.expect_string(".aligned")?;
            let aligned = ();
            stream.expect_complete()?;
            let dst = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let src = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let size = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(
                TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
                    cp_fenceproxy,
                    cp_qualifiers,
                    fence_qualifiers,
                    sync,
                    aligned,
                    dst,
                    src,
                    size,
                },
            )
        }
    }
}
