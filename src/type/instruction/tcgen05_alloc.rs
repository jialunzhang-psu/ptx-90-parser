//! Original PTX specification:
//!
//! tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;
//! tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;
//! tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;
//! .cta_group = { .cta_group::1, .cta_group::2 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 {
        pub alloc: (),             // .alloc
        pub cta_group: CtaGroup,   // .cta_group
        pub sync: (),              // .sync
        pub aligned: (),           // .aligned
        pub shared_cta: bool,      // {.shared::cta}
        pub b32: (),               // .b32
        pub dst: AddressOperand,   // [dst]
        pub ncols: GeneralOperand, // nCols
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05DeallocCtaGroupSyncAlignedB32 {
        pub dealloc: (),           // .dealloc
        pub cta_group: CtaGroup,   // .cta_group
        pub sync: (),              // .sync
        pub aligned: (),           // .aligned
        pub b32: (),               // .b32
        pub taddr: GeneralOperand, // taddr
        pub ncols: GeneralOperand, // nCols
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05RelinquishAllocPermitCtaGroupSyncAligned {
        pub relinquish_alloc_permit: (), // .relinquish_alloc_permit
        pub cta_group: CtaGroup,         // .cta_group
        pub sync: (),                    // .sync
        pub aligned: (),                 // .aligned
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CtaGroup as CtaGroup0;
pub use section_0::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32;
pub use section_0::Tcgen05DeallocCtaGroupSyncAlignedB32;
pub use section_0::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned;
