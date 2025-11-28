//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem],  a-desc,  b-desc,  idesc,
//! enable-input-d {, zero-column-mask-desc };
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc };
//! .kind = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! ----------------------------------------------------------------------------------
//! // 2. Integer type:
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem],  a-desc,  b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .collector_usage = { .collector::buffer::op };
//! ::buffer = { ::b0, ::b1, ::b2, ::b3 };
//! ::op   = { ::fill, ::use, ::lastuse, ::discard};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
        KindTf32, // .kind::tf32
        KindF16, // .kind::f16
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05MmaWsCtaGroup1KindCollectorUsage {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub cta_group_1: (), // .cta_group::1
        pub kind: Kind, // .kind
        pub collector_usage: bool, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_desc: GeneralOperand, // a-desc
        pub b_desc: GeneralOperand, // b-desc
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05MmaWsCtaGroup1KindCollectorUsage1 {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub cta_group_1: (), // .cta_group::1
        pub kind: Kind, // .kind
        pub collector_usage: bool, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_tmem: AddressOperand, // [a-tmem]
        pub b_desc: GeneralOperand, // b-desc
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
        pub span: Span,
    }

}

pub mod section_1 {
    use crate::r#type::common::*;
    use crate::parser::Span;
    use crate::Spanned;

    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Buffer {
        B0, // ::b0
        B1, // ::b1
        B2, // ::b2
        B3, // ::b3
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum Op {
        Lastuse, // ::lastuse
        Discard, // ::discard
        Fill, // ::fill
        Use, // ::use
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum CollectorUsage {
        CollectorBufferOp((), Buffer, Op), // .collector::buffer::op
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05MmaWsCtaGroup1KindI8CollectorUsage {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub cta_group_1: (), // .cta_group::1
        pub kind_i8: (), // .kind::i8
        pub collector_usage: Option<CollectorUsage>, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_desc: GeneralOperand, // a-desc
        pub b_desc: GeneralOperand, // b-desc
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
    pub struct Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub cta_group_1: (), // .cta_group::1
        pub kind_i8: (), // .kind::i8
        pub collector_usage: Option<CollectorUsage>, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_tmem: AddressOperand, // [a-tmem]
        pub b_desc: GeneralOperand, // b-desc
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
        pub span: Span,
    }

}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage;
pub use section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage1;
pub use section_0::Kind as Kind0;
pub use section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage;
pub use section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1;
pub use section_1::Buffer as Buffer1;
pub use section_1::Op as Op1;
pub use section_1::CollectorUsage as CollectorUsage1;
