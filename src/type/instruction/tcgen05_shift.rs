//! Original PTX specification:
//!
//! tcgen05.shift.cta_group.down  [taddr];
//! .cta_group = { .cta_group::1, .cta_group::2 }

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
    pub struct Tcgen05ShiftCtaGroupDown {
        pub shift: (),             // .shift
        pub cta_group: CtaGroup,   // .cta_group
        pub down: (),              // .down
        pub taddr: AddressOperand, // [taddr]
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CtaGroup as CtaGroup0;
pub use section_0::Tcgen05ShiftCtaGroupDown;
