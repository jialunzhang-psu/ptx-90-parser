//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.ws.sp.cta_group::1.kind{.collector_usage} [d-tmem],  a-desc,  b-desc,
//! [sp-meta-tmem] ,  idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.sp.cta_group::1.kind{.collector_usage} [d-tmem], [a-tmem], b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .kind = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! ------------------------------------------------------------------
//! // 2. Integer type:
//! tcgen05.mma.ws.sp.cta_group::1.kind::i8{.collector_usage} [d-tmem], a-desc, b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.sp.cta_group::1.kind::i8{.collector_usage} [d-tmem], [a-tmem], b-desc,
//! [sp-meta-tmem] , idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .collector_usage = { .collector::buffer::op };
//! ::buffer = { ::b0, ::b1, ::b2, ::b3 };
//! ::op   = { ::fill, ::use, ::lastuse, ::discard};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
        KindTf32, // .kind::tf32
        KindF16, // .kind::f16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05MmaWsSpCtaGroup1KindCollectorUsage {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub sp: (), // .sp
        pub cta_group_1: (), // .cta_group::1
        pub kind: Kind, // .kind
        pub collector_usage: bool, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_desc: GeneralOperand, // a-desc
        pub b_desc: GeneralOperand, // b-desc
        pub sp_meta_tmem: AddressOperand, // [sp-meta-tmem]
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1 {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub sp: (), // .sp
        pub cta_group_1: (), // .cta_group::1
        pub kind: Kind, // .kind
        pub collector_usage: bool, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_tmem: AddressOperand, // [a-tmem]
        pub b_desc: GeneralOperand, // b-desc
        pub sp_meta_tmem: AddressOperand, // [sp-meta-tmem]
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
    }

}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Buffer {
        B0, // ::b0
        B1, // ::b1
        B2, // ::b2
        B3, // ::b3
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Lastuse, // ::lastuse
        Discard, // ::discard
        Fill, // ::fill
        Use, // ::use
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CollectorUsage {
        CollectorBufferOp((), Buffer, Op), // .collector::buffer::op
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub sp: (), // .sp
        pub cta_group_1: (), // .cta_group::1
        pub kind_i8: (), // .kind::i8
        pub collector_usage: Option<CollectorUsage>, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_desc: GeneralOperand, // a-desc
        pub b_desc: GeneralOperand, // b-desc
        pub sp_meta_tmem: AddressOperand, // [sp-meta-tmem]
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1 {
        pub mma: (), // .mma
        pub ws: (), // .ws
        pub sp: (), // .sp
        pub cta_group_1: (), // .cta_group::1
        pub kind_i8: (), // .kind::i8
        pub collector_usage: Option<CollectorUsage>, // {.collector_usage}
        pub d_tmem: AddressOperand, // [d-tmem]
        pub a_tmem: AddressOperand, // [a-tmem]
        pub b_desc: GeneralOperand, // b-desc
        pub sp_meta_tmem: AddressOperand, // [sp-meta-tmem]
        pub idesc: GeneralOperand, // idesc
        pub enable_input_d: GeneralOperand, // enable-input-d
        pub zero_column_mask_desc: Option<GeneralOperand>, // {, zero-column-mask-desc}
    }

}
