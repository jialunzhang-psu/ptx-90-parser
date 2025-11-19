//! Original PTX specification:
//!
//! tensormap.cp_fenceproxy.cp_qualifiers.fence_qualifiers.sync.aligned  [dst], [src], size;
//! .cp_qualifiers    = { .global.shared::cta };
//! .fence_qualifiers = { .to_proxy::from_proxy.release.scope };
//! .to_proxy::from_proxy  = { .tensormap::generic };
//! .scope            = { .cta, .cluster, .gpu , .sys };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CpQualifiers {
        GlobalSharedCta, // .global.shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ToProxyFromProxy {
        TensormapGeneric, // .tensormap::generic
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Scope {
        Cluster, // .cluster
        Cta,     // .cta
        Gpu,     // .gpu
        Sys,     // .sys
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum FenceQualifiers {
        ToProxyFromProxyReleaseScope(ToProxyFromProxy, (), Scope), // .to_proxy::from_proxy.release.scope
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        pub cp_fenceproxy: (),                 // .cp_fenceproxy
        pub cp_qualifiers: CpQualifiers,       // .cp_qualifiers
        pub fence_qualifiers: FenceQualifiers, // .fence_qualifiers
        pub sync: (),                          // .sync
        pub aligned: (),                       // .aligned
        pub dst: AddressOperand,               // [dst]
        pub src: AddressOperand,               // [src]
        pub size: GeneralOperand,              // size
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CpQualifiers as CpQualifiers0;
pub use section_0::FenceQualifiers as FenceQualifiers0;
pub use section_0::Scope as Scope0;
pub use section_0::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned;
pub use section_0::ToProxyFromProxy as ToProxyFromProxy0;
