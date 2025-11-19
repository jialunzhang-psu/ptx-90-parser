//! Original PTX specification:
//!
//! tensormap.cp_fenceproxy.cp_qualifiers.fence_qualifiers.sync.aligned  [dst], [src], size;
//! .cp_qualifiers    = { .global.shared::cta };
//! .fence_qualifiers = { .to_proxy::from_proxy.release.scope };
//! .to_proxy::from_proxy  = { .tensormap::generic };
//! .scope            = { .cta, .cluster, .gpu , .sys };

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
    use crate::r#type::instruction::tensormap_cp_fenceproxy::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CpQualifiers {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global.shared::cta"), |_, _span| {
                CpQualifiers::GlobalSharedCta
            }))
        }
    }

    impl PtxParser for FenceQualifiers {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                seq_n!(
                    ToProxyFromProxy::parse(),
                    string_p(".release"),
                    Scope::parse()
                ),
                |(to_proxy_from_proxy, release, scope), _span| {
                    FenceQualifiers::ToProxyFromProxyReleaseScope(
                        to_proxy_from_proxy,
                        release,
                        scope,
                    )
                }
            ))
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".cluster"), |_, _span| Scope::Cluster),
                map(string_p(".cta"), |_, _span| Scope::Cta),
                map(string_p(".gpu"), |_, _span| Scope::Gpu),
                map(string_p(".sys"), |_, _span| Scope::Sys)
            )
        }
    }

    impl PtxParser for ToProxyFromProxy {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tensormap::generic"), |_, _span| {
                ToProxyFromProxy::TensormapGeneric
            }))
        }
    }

    impl PtxParser for TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tensormap"),
                    string_p(".cp_fenceproxy"),
                    CpQualifiers::parse(),
                    FenceQualifiers::parse(),
                    string_p(".sync"),
                    string_p(".aligned"),
                    AddressOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    cp_fenceproxy,
                    cp_qualifiers,
                    fence_qualifiers,
                    sync,
                    aligned,
                    dst,
                    _,
                    src,
                    _,
                    size,
                    _,
                ),
                 span| {
                    ok!(TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
                        cp_fenceproxy = cp_fenceproxy,
                        cp_qualifiers = cp_qualifiers,
                        fence_qualifiers = fence_qualifiers,
                        sync = sync,
                        aligned = aligned,
                        dst = dst,
                        src = src,
                        size = size,

                    })
                },
            )
        }
    }
}
