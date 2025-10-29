use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;`
/// `tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;`
/// `tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tcgen05 {
    /// `tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;`
    Alloc(Alloc),
    /// `tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;`
    Dealloc(Dealloc),
    /// `tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;`
    RelinquishAllocPermit(RelinquishAllocPermit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alloc {
    /// `.cta_group`
    pub cta_group: CtaGroup,
    /// `.shared::cta`
    pub state_space: Option<StateSpace>,
    /// `[dst]`
    pub destination: AddressOperand,
    /// `nCols`
    pub column_count: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dealloc {
    /// `.cta_group`
    pub cta_group: CtaGroup,
    /// `taddr`
    pub tensor_address: RegisterOperand,
    /// `nCols`
    pub column_count: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelinquishAllocPermit {
    /// `.cta_group`
    pub cta_group: CtaGroup,
}

/// `.cta_group = { .cta_group::1, .cta_group::2 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtaGroup {
    /// `.cta_group::1`
    One,
    /// `.cta_group::2`
    Two,
}

/// `.shared::cta`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.shared::cta`
    SharedCta,
}
