//! Original PTX specification:
//!
//! tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;
//! tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;
//! tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;
//! .cta_group = { .cta_group::1, .cta_group::2 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
        pub alloc: (), // .alloc
        pub cta_group: CtaGroup, // .cta_group
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub shared_cta: bool, // {.shared::cta}
        pub b32: (), // .b32
        pub dst: AddressOperand, // [dst]
        pub ncols: Operand, // nCols
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05DeallocCtaGroupSyncAlignedB32 {
        pub dealloc: (), // .dealloc
        pub cta_group: CtaGroup, // .cta_group
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub b32: (), // .b32
        pub taddr: Operand, // taddr
        pub ncols: Operand, // nCols
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
        pub relinquish_alloc_permit: (), // .relinquish_alloc_permit
        pub cta_group: CtaGroup, // .cta_group
        pub sync: (), // .sync
        pub aligned: (), // .aligned
    }

}
