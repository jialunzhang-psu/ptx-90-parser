//! Original PTX specification:
//!
//! barrier{.cta}.sync{.aligned}      a{, b};
//! barrier{.cta}.arrive{.aligned}    a, b;
//! barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
//! barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
//! bar{.cta}.sync      a{, b};
//! bar{.cta}.arrive    a, b;
//! bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
//! bar{.cta}.red.op.pred   p, a{, b}, {!}c;
//! .op = { .and, .or };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        And, // .and
        Or,  // .or
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierCtaSyncAligned {
        pub cta: bool,                 // {.cta}
        pub sync: (),                  // .sync
        pub aligned: bool,             // {.aligned}
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierCtaArriveAligned {
        pub cta: bool,         // {.cta}
        pub arrive: (),        // .arrive
        pub aligned: bool,     // {.aligned}
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierCtaRedPopcAlignedU32 {
        pub cta: bool,                 // {.cta}
        pub red: (),                   // .red
        pub popc: (),                  // .popc
        pub aligned: bool,             // {.aligned}
        pub u32: (),                   // .u32
        pub d: GeneralOperand,         // d
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
        pub c_op: bool,                // {!} operator
        pub c: GeneralOperand,         // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarrierCtaRedOpAlignedPred {
        pub cta: bool,                 // {.cta}
        pub red: (),                   // .red
        pub op: Op,                    // .op
        pub aligned: bool,             // {.aligned}
        pub pred: (),                  // .pred
        pub p: GeneralOperand,         // p
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
        pub c_op: bool,                // {!} operator
        pub c: GeneralOperand,         // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarCtaSync {
        pub cta: bool,                 // {.cta}
        pub sync: (),                  // .sync
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarCtaArrive {
        pub cta: bool,         // {.cta}
        pub arrive: (),        // .arrive
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarCtaRedPopcU32 {
        pub cta: bool,                 // {.cta}
        pub red: (),                   // .red
        pub popc: (),                  // .popc
        pub u32: (),                   // .u32
        pub d: GeneralOperand,         // d
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
        pub c_op: bool,                // {!} operator
        pub c: GeneralOperand,         // {!}c
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BarCtaRedOpPred {
        pub cta: bool,                 // {.cta}
        pub red: (),                   // .red
        pub op: Op,                    // .op
        pub pred: (),                  // .pred
        pub p: GeneralOperand,         // p
        pub a: GeneralOperand,         // a
        pub b: Option<GeneralOperand>, // {, b}
        pub c_op: bool,                // {!} operator
        pub c: GeneralOperand,         // {!}c
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::BarCtaArrive;
pub use section_0::BarCtaRedOpPred;
pub use section_0::BarCtaRedPopcU32;
pub use section_0::BarCtaSync;
pub use section_0::BarrierCtaArriveAligned;
pub use section_0::BarrierCtaRedOpAlignedPred;
pub use section_0::BarrierCtaRedPopcAlignedU32;
pub use section_0::BarrierCtaSyncAligned;
pub use section_0::Op as Op0;
