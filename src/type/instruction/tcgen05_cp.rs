//! Original PTX specification:
//!
//! tcgen05.cp.cta_group.shape{.multicast}{.dst_src_fmt} [taddr], s-desc;
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! .dst_src_fmt   = { .b8x16.b6x16_p32, .b8x16.b4x16_p64 };
//! .shape     = { .128x256b, .4x256b, .128x128b, .64x128b**, .32x128b*** };
//! .multicast = { .warpx2::02_13** , .warpx2::01_23**, .warpx4*** };

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

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        _32x128b,  // .32x128b***
        _64x128b,  // .64x128b**
        _128x256b, // .128x256b
        _128x128b, // .128x128b
        _4x256b,   // .4x256b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Multicast {
        Warpx20213, // .warpx2::02_13**
        Warpx20123, // .warpx2::01_23**
        Warpx4,     // .warpx4***
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DstSrcFmt {
        B8x16B6x16P32, // .b8x16.b6x16_p32
        B8x16B4x16P64, // .b8x16.b4x16_p64
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
        pub cp: (),                         // .cp
        pub cta_group: CtaGroup,            // .cta_group
        pub shape: Shape,                   // .shape
        pub multicast: Option<Multicast>,   // {.multicast}
        pub dst_src_fmt: Option<DstSrcFmt>, // {.dst_src_fmt}
        pub taddr: AddressOperand,          // [taddr]
        pub s_desc: GeneralOperand,         // s-desc
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CtaGroup as CtaGroup0;
pub use section_0::DstSrcFmt as DstSrcFmt0;
pub use section_0::Multicast as Multicast0;
pub use section_0::Shape as Shape0;
pub use section_0::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt;
