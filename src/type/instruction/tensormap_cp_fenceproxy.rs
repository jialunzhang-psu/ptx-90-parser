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
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CpQualifiers {
        GlobalSharedCta, // .global.shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum FenceQualifiers {
        ToProxyFromProxyReleaseScope, // .to_proxy::from_proxy.release.scope
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned {
        pub cp_fenceproxy: (), // .cp_fenceproxy
        pub cp_qualifiers: CpQualifiers, // .cp_qualifiers
        pub fence_qualifiers: FenceQualifiers, // .fence_qualifiers
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub dst: AddressOperand, // [dst]
        pub src: AddressOperand, // [src]
        pub size: Operand, // size
    }

}
