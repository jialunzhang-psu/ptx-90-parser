//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.sp.cta_group.kind  [d-tmem],  a-desc,  b-desc, [sp-meta-tmem] ,  idesc {, disable-output-lane }, enable-input-d{, scale-input-d};
//! tcgen05.mma.sp.cta_group.kind  [d-tmem], [a-tmem], b-desc, [sp-meta-tmem] , idesc {, disable-output-lane }, enable-input-d{, scale-input-d};
//! .kind       = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group  = { .cta_group::1,  .cta_group::2 };
//! ------------------------------------------------------------------
//! // 2. Floating-point type with block scaling:
//! tcgen05.mma.sp.cta_group.kind.block_scale{.scale_vectorsize} [d-tmem],  a-desc,  b-desc , [sp-meta-tmem] , idesc, [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.sp.cta_group.kind.block_scale{.scale_vectorsize} [d-tmem], [a-tmem], b-desc , [sp-meta-tmem] , idesc, [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .scale_vectorsize = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! .cta_group      = { .cta_group::1,  .cta_group::2 };
//! .kind = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! ------------------------------------------------------------------
//! // 3. Convolution MMA with floating-point type without block scaling:
//! tcgen05.mma.sp.cta_group.kind.collector_usage           [d-tmem],  a-desc,  b-desc, [sp-meta-tmem] ,  idesc {, disable-output-lane }, enable-input-d
//! {, scale-input-d};
//! tcgen05.mma.sp.cta_group.kind.ashift{.collector_usage}  [d-tmem], [a-tmem], b-desc, [sp-meta-tmem] , idesc {, disable-output-lane }, enable-input-d
//! {, scale-input-d};
//! tcgen05.mma.sp.cta_group.kind{.ashift}.collector_usage  [d-tmem], [a-tmem], b-desc, [sp-meta-tmem] , idesc {, disable-output-lane }, enable-input-d
//! {, scale-input-d};
//! .kind            = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 4. Activation Stationary MMA with floating-point type with block scaling:
//! tcgen05.mma.sp.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage [d-tmem],  a-desc,  b-desc , [sp-meta-tmem] , idesc, [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.sp.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage [d-tmem], [a-tmem], b-desc , [sp-meta-tmem] , idesc, [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .kind = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .scale_vectorsize = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 5. Integer type:
//! tcgen05.mma.sp.cta_group.kind::i8 [d-tmem],  a-desc,  b-desc, [sp-meta-tmem] , idesc {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.sp.cta_group.kind::i8 [d-tmem], [a-tmem], b-desc, [sp-meta-tmem] , idesc {, disable-output-lane }, enable-input-d;
//! .cta_group      = { .cta_group::1,  .cta_group::2 };
//! ------------------------------------------------------------------
//! // 6. Convolution MMA with Integer type:
//! tcgen05.mma.sp.cta_group.kind::i8.collector_usage          [d-tmem],  a-desc,  b-desc, [sp-meta-tmem], idesc {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.sp.cta_group.kind::i8.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, [sp-meta-tmem], idesc {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.sp.cta_group.kind::i8{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, [sp-meta-tmem], idesc {, disable-output-lane }, enable-input-d;
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };

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
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
        KindTf32,   // .kind::tf32
        KindF16,    // .kind::f16
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKind {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind: Kind,                                  // .kind
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_desc: GeneralOperand,                      // a-desc
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub scale_input_d: Option<GeneralOperand>,       // {, scale-input-d}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKind1 {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind: Kind,                                  // .kind
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub scale_input_d: Option<GeneralOperand>,       // {, scale-input-d}
        pub span: Span,
    }
}

pub mod section_1 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Kind {
        KindMxf8f6f4, // .kind::mxf8f6f4
        KindMxf4nvf4, // .kind::mxf4nvf4
        KindMxf4,     // .kind::mxf4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ScaleVectorsize {
        ScaleVec1x, // .scale_vec::1X
        ScaleVec2x, // .scale_vec::2X
        ScaleVec4x, // .scale_vec::4X
        Block16,    // .block16
        Block32,    // .block32
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize {
        pub mma: (),                                   // .mma
        pub sp: (),                                    // .sp
        pub cta_group: CtaGroup,                       // .cta_group
        pub kind: Kind,                                // .kind
        pub block_scale: (),                           // .block_scale
        pub scale_vectorsize: Option<ScaleVectorsize>, // {.scale_vectorsize}
        pub d_tmem: AddressOperand,                    // [d-tmem]
        pub a_desc: GeneralOperand,                    // a-desc
        pub b_desc: GeneralOperand,                    // b-desc
        pub sp_meta_tmem: AddressOperand,              // [sp-meta-tmem]
        pub idesc: GeneralOperand,                     // idesc
        pub scale_a_tmem: AddressOperand,              // [scale-A-tmem]
        pub scale_b_tmem: AddressOperand,              // [scale-B-tmem]
        pub enable_input_d: GeneralOperand,            // enable-input-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1 {
        pub mma: (),                                   // .mma
        pub sp: (),                                    // .sp
        pub cta_group: CtaGroup,                       // .cta_group
        pub kind: Kind,                                // .kind
        pub block_scale: (),                           // .block_scale
        pub scale_vectorsize: Option<ScaleVectorsize>, // {.scale_vectorsize}
        pub d_tmem: AddressOperand,                    // [d-tmem]
        pub a_tmem: AddressOperand,                    // [a-tmem]
        pub b_desc: GeneralOperand,                    // b-desc
        pub sp_meta_tmem: AddressOperand,              // [sp-meta-tmem]
        pub idesc: GeneralOperand,                     // idesc
        pub scale_a_tmem: AddressOperand,              // [scale-A-tmem]
        pub scale_b_tmem: AddressOperand,              // [scale-B-tmem]
        pub enable_input_d: GeneralOperand,            // enable-input-d
        pub span: Span,
    }
}

pub mod section_2 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Kind {
        KindF8f6f4, // .kind::f8f6f4
        KindTf32,   // .kind::tf32
        KindF16,    // .kind::f16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Buffer {
        A, // ::a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Discard, // ::discard*
        Lastuse, // ::lastuse
        Fill,    // ::fill
        Use,     // ::use
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CollectorUsage {
        CollectorBufferOp((), Buffer, Op), // .collector::buffer::op
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindCollectorUsage {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind: Kind,                                  // .kind
        pub collector_usage: CollectorUsage,             // .collector_usage
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_desc: GeneralOperand,                      // a-desc
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub scale_input_d: Option<GeneralOperand>,       // {, scale-input-d}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind: Kind,                                  // .kind
        pub ashift: (),                                  // .ashift
        pub collector_usage: Option<CollectorUsage>,     // {.collector_usage}
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub scale_input_d: Option<GeneralOperand>,       // {, scale-input-d}
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1 {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind: Kind,                                  // .kind
        pub ashift: bool,                                // {.ashift}
        pub collector_usage: CollectorUsage,             // .collector_usage
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub scale_input_d: Option<GeneralOperand>,       // {, scale-input-d}
        pub span: Span,
    }
}

pub mod section_3 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Kind {
        KindMxf8f6f4, // .kind::mxf8f6f4
        KindMxf4nvf4, // .kind::mxf4nvf4
        KindMxf4,     // .kind::mxf4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ScaleVectorsize {
        ScaleVec1x, // .scale_vec::1X
        ScaleVec2x, // .scale_vec::2X
        ScaleVec4x, // .scale_vec::4X
        Block16,    // .block16
        Block32,    // .block32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Buffer {
        A, // ::a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Discard, // ::discard*
        Lastuse, // ::lastuse
        Fill,    // ::fill
        Use,     // ::use
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CollectorUsage {
        CollectorBufferOp((), Buffer, Op), // .collector::buffer::op
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
        pub mma: (),                                   // .mma
        pub sp: (),                                    // .sp
        pub cta_group: CtaGroup,                       // .cta_group
        pub kind: Kind,                                // .kind
        pub block_scale: (),                           // .block_scale
        pub scale_vectorsize: Option<ScaleVectorsize>, // {.scale_vectorsize}
        pub collector_usage: CollectorUsage,           // .collector_usage
        pub d_tmem: AddressOperand,                    // [d-tmem]
        pub a_desc: GeneralOperand,                    // a-desc
        pub b_desc: GeneralOperand,                    // b-desc
        pub sp_meta_tmem: AddressOperand,              // [sp-meta-tmem]
        pub idesc: GeneralOperand,                     // idesc
        pub scale_a_tmem: AddressOperand,              // [scale-A-tmem]
        pub scale_b_tmem: AddressOperand,              // [scale-B-tmem]
        pub enable_input_d: GeneralOperand,            // enable-input-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
        pub mma: (),                                   // .mma
        pub sp: (),                                    // .sp
        pub cta_group: CtaGroup,                       // .cta_group
        pub kind: Kind,                                // .kind
        pub block_scale: (),                           // .block_scale
        pub scale_vectorsize: Option<ScaleVectorsize>, // {.scale_vectorsize}
        pub collector_usage: CollectorUsage,           // .collector_usage
        pub d_tmem: AddressOperand,                    // [d-tmem]
        pub a_tmem: AddressOperand,                    // [a-tmem]
        pub b_desc: GeneralOperand,                    // b-desc
        pub sp_meta_tmem: AddressOperand,              // [sp-meta-tmem]
        pub idesc: GeneralOperand,                     // idesc
        pub scale_a_tmem: AddressOperand,              // [scale-A-tmem]
        pub scale_b_tmem: AddressOperand,              // [scale-B-tmem]
        pub enable_input_d: GeneralOperand,            // enable-input-d
        pub span: Span,
    }
}

pub mod section_4 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindI8 {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind_i8: (),                                 // .kind::i8
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_desc: GeneralOperand,                      // a-desc
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindI81 {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind_i8: (),                                 // .kind::i8
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub span: Span,
    }
}

pub mod section_5 {
    use crate::Spanned;
    use crate::parser::Span;
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CtaGroup {
        CtaGroup1, // .cta_group::1
        CtaGroup2, // .cta_group::2
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Buffer {
        A, // ::a
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Op {
        Discard, // ::discard*
        Lastuse, // ::lastuse
        Fill,    // ::fill
        Use,     // ::use
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CollectorUsage {
        CollectorBufferOp((), Buffer, Op), // .collector::buffer::op
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindI8CollectorUsage {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind_i8: (),                                 // .kind::i8
        pub collector_usage: CollectorUsage,             // .collector_usage
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_desc: GeneralOperand,                      // a-desc
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind_i8: (),                                 // .kind::i8
        pub ashift: (),                                  // .ashift
        pub collector_usage: Option<CollectorUsage>,     // {.collector_usage}
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub span: Span,
    }

    #[derive(Debug, Clone, PartialEq, Spanned)]
    pub struct Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1 {
        pub mma: (),                                     // .mma
        pub sp: (),                                      // .sp
        pub cta_group: CtaGroup,                         // .cta_group
        pub kind_i8: (),                                 // .kind::i8
        pub ashift: bool,                                // {.ashift}
        pub collector_usage: CollectorUsage,             // .collector_usage
        pub d_tmem: AddressOperand,                      // [d-tmem]
        pub a_tmem: AddressOperand,                      // [a-tmem]
        pub b_desc: GeneralOperand,                      // b-desc
        pub sp_meta_tmem: AddressOperand,                // [sp-meta-tmem]
        pub idesc: GeneralOperand,                       // idesc
        pub disable_output_lane: Option<GeneralOperand>, // {, disable-output-lane}
        pub enable_input_d: GeneralOperand,              // enable-input-d
        pub span: Span,
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::CtaGroup as CtaGroup0;
pub use section_0::Kind as Kind0;
pub use section_0::Tcgen05MmaSpCtaGroupKind;
pub use section_0::Tcgen05MmaSpCtaGroupKind1;
pub use section_1::CtaGroup as CtaGroup1;
pub use section_1::Kind as Kind1;
pub use section_1::ScaleVectorsize as ScaleVectorsize1;
pub use section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize;
pub use section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1;
pub use section_2::Buffer as Buffer2;
pub use section_2::CollectorUsage as CollectorUsage2;
pub use section_2::CtaGroup as CtaGroup2;
pub use section_2::Kind as Kind2;
pub use section_2::Op as Op2;
pub use section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage;
pub use section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1;
pub use section_2::Tcgen05MmaSpCtaGroupKindCollectorUsage;
pub use section_3::Buffer as Buffer3;
pub use section_3::CollectorUsage as CollectorUsage3;
pub use section_3::CtaGroup as CtaGroup3;
pub use section_3::Kind as Kind3;
pub use section_3::Op as Op3;
pub use section_3::ScaleVectorsize as ScaleVectorsize3;
pub use section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage;
pub use section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1;
pub use section_4::CtaGroup as CtaGroup4;
pub use section_4::Tcgen05MmaSpCtaGroupKindI8;
pub use section_4::Tcgen05MmaSpCtaGroupKindI81;
pub use section_5::Buffer as Buffer5;
pub use section_5::CollectorUsage as CollectorUsage5;
pub use section_5::CtaGroup as CtaGroup5;
pub use section_5::Op as Op5;
pub use section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage;
pub use section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1;
pub use section_5::Tcgen05MmaSpCtaGroupKindI8CollectorUsage;
