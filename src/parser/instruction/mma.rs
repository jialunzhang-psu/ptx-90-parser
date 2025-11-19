//! Original PTX specification:
//!
//! // Half precision floating point type:
//! mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .ctype   = {.f16, .f32};
//! .dtype   = {.f16, .f32};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32        d, a, b, c;
//! mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32      d, a, b, c;
//! mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32       d, a, b, c;
//! mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype  d, a, b, c;
//! mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;
//! .atype      = {.bf16, .tf32};
//! .btype      = {.bf16, .tf32};
//! .f8type     = {.e4m3, .e5m2};
//! .f8f6f4type = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .ctype      = {.f16, .f32};
//! .dtype      = {.f16, .f32};
//! .shape      = {.m16n8k16, .m16n8k32};
//! .kind       = {.kind::f8f6f4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Alternate floating point type with block scaling:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4};
//! .scale_vec_size = {.scale_vec::2X};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf4nvf4};
//! .scale_vec_size = {.scale_vec::2X, .scale_vec::4X};
//! .stype          = {.ue8m0, .ue4m3};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};
//! .kind           = {.kind::mxf8f6f4};
//! .scale_vec_size = {.scale_vec::1X};
//! .f8f6f4type     = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
//! .stype          = {.ue8m0};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Double precision floating point type:
//! mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;
//! .shape   = {.m8n84, .m16n8k4, .m16n8k8, .m16n8k16};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Integer type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k16, .m16n8k16, .m16n8k32};
//! .atype   = {.u8, .s8};
//! .btype   = {.u8, .s8};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;
//! .shape   = {.m8n8k32, .m16n8k32, .m16n8k64};
//! .atype   = {.u4, .s4};
//! .btype   = {.u4, .s4};
//! ----------------------------------------------------
//! // Alternate floating point type:
//! // Single bit:
//! mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;
//! .bitOp = {.xor, .and};
//! .shape = {.m8n8k128, .m16n8k128, .m16n8k256};

#![allow(unused)]

use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{
        between, comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, map, minus_p, optional,
        pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, try_map,
    },
};
use crate::r#type::common::*;
use crate::{alt, ok, seq_n};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::mma::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Alayout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Alayout::Row),
                map(string_p(".col"), |_, _span| Alayout::Col)
            )
        }
    }

    impl PtxParser for Blayout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Blayout::Row),
                map(string_p(".col"), |_, _span| Blayout::Col)
            )
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Ctype::F16),
                map(string_p(".f32"), |_, _span| Ctype::F32)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m8n8k4"),
                    Alayout::parse(),
                    Blayout::parse(),
                    Dtype::parse(),
                    string_p(".f16"),
                    string_p(".f16"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m8n8k4,
                    alayout,
                    blayout,
                    dtype,
                    f16,
                    f162,
                    ctype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype {
                        sync = sync,
                        aligned = aligned,
                        m8n8k4 = m8n8k4,
                        alayout = alayout,
                        blayout = blayout,
                        dtype = dtype,
                        f16 = f16,
                        f162 = f162,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k8"),
                    string_p(".row"),
                    string_p(".col"),
                    Dtype::parse(),
                    string_p(".f16"),
                    string_p(".f16"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k8,
                    row,
                    col,
                    dtype,
                    f16,
                    f162,
                    ctype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype {
                        sync = sync,
                        aligned = aligned,
                        m16n8k8 = m16n8k8,
                        row = row,
                        col = col,
                        dtype = dtype,
                        f16 = f16,
                        f162 = f162,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k16"),
                    string_p(".row"),
                    string_p(".col"),
                    Dtype::parse(),
                    string_p(".f16"),
                    string_p(".f16"),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k16,
                    row,
                    col,
                    dtype,
                    f16,
                    f162,
                    ctype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype {
                        sync = sync,
                        aligned = aligned,
                        m16n8k16 = m16n8k16,
                        row = row,
                        col = col,
                        dtype = dtype,
                        f16 = f16,
                        f162 = f162,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::mma::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Atype::Bf16),
                map(string_p(".tf32"), |_, _span| Atype::Tf32)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".bf16"), |_, _span| Btype::Bf16),
                map(string_p(".tf32"), |_, _span| Btype::Tf32)
            )
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Ctype::F16),
                map(string_p(".f32"), |_, _span| Ctype::F32)
            )
        }
    }

    impl PtxParser for Dtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Dtype::F16),
                map(string_p(".f32"), |_, _span| Dtype::F32)
            )
        }
    }

    impl PtxParser for F8f6f4type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3"), |_, _span| F8f6f4type::E4m3),
                map(string_p(".e5m2"), |_, _span| F8f6f4type::E5m2),
                map(string_p(".e3m2"), |_, _span| F8f6f4type::E3m2),
                map(string_p(".e2m3"), |_, _span| F8f6f4type::E2m3),
                map(string_p(".e2m1"), |_, _span| F8f6f4type::E2m1)
            )
        }
    }

    impl PtxParser for F8type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3"), |_, _span| F8type::E4m3),
                map(string_p(".e5m2"), |_, _span| F8type::E5m2)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".kind::f8f6f4"), |_, _span| Kind::KindF8f6f4))
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8k16"), |_, _span| Shape::M16n8k16),
                map(string_p(".m16n8k32"), |_, _span| Shape::M16n8k32)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k4"),
                    string_p(".row"),
                    string_p(".col"),
                    string_p(".f32"),
                    string_p(".tf32"),
                    string_p(".tf32"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k4,
                    row,
                    col,
                    f32,
                    tf32,
                    tf322,
                    f322,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 {
                        sync = sync,
                        aligned = aligned,
                        m16n8k4 = m16n8k4,
                        row = row,
                        col = col,
                        f32 = f32,
                        tf32 = tf32,
                        tf322 = tf322,
                        f322 = f322,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k8"),
                    string_p(".row"),
                    string_p(".col"),
                    string_p(".f32"),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k8,
                    row,
                    col,
                    f32,
                    atype,
                    btype,
                    f322,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 {
                        sync = sync,
                        aligned = aligned,
                        m16n8k8 = m16n8k8,
                        row = row,
                        col = col,
                        f32 = f32,
                        atype = atype,
                        btype = btype,
                        f322 = f322,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k16"),
                    string_p(".row"),
                    string_p(".col"),
                    string_p(".f32"),
                    string_p(".bf16"),
                    string_p(".bf16"),
                    string_p(".f32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k16,
                    row,
                    col,
                    f32,
                    bf16,
                    bf162,
                    f322,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 {
                        sync = sync,
                        aligned = aligned,
                        m16n8k16 = m16n8k16,
                        row = row,
                        col = col,
                        f32 = f32,
                        bf16 = bf16,
                        bf162 = bf162,
                        f322 = f322,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    string_p(".row"),
                    string_p(".col"),
                    Dtype::parse(),
                    F8type::parse(),
                    F8type::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    shape,
                    row,
                    col,
                    dtype,
                    f8type,
                    f8type1,
                    ctype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        row = row,
                        col = col,
                        dtype = dtype,
                        f8type = f8type,
                        f8type1 = f8type1,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k32"),
                    string_p(".row"),
                    string_p(".col"),
                    Kind::parse(),
                    Dtype::parse(),
                    F8f6f4type::parse(),
                    F8f6f4type::parse(),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k32,
                    row,
                    col,
                    kind,
                    dtype,
                    f8f6f4type,
                    f8f6f4type1,
                    ctype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype {
                        sync = sync,
                        aligned = aligned,
                        m16n8k32 = m16n8k32,
                        row = row,
                        col = col,
                        kind = kind,
                        dtype = dtype,
                        f8f6f4type = f8f6f4type,
                        f8f6f4type1 = f8f6f4type1,
                        ctype = ctype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::mma::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".kind::mxf4"), |_, _span| Kind::KindMxf4))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".scale_vec::2X"), |_, _span| {
                ScaleVecSize::ScaleVec2x
            }))
        }
    }

    impl PtxParser for Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".ue8m0"), |_, _span| Stype::Ue8m0))
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k64"),
                    string_p(".row"),
                    string_p(".col"),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVecSize::parse()),
                    string_p(".f32"),
                    string_p(".e2m1"),
                    string_p(".e2m1"),
                    string_p(".f32"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k64,
                    row,
                    col,
                    kind,
                    block_scale,
                    scale_vec_size,
                    f32,
                    e2m1,
                    e2m12,
                    f322,
                    stype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                    scale_a_data,
                    _,
                    byte_id_a,
                    _,
                    scale_b_data,
                    _,
                    byte_id_b,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype {
                        sync = sync,
                        aligned = aligned,
                        m16n8k64 = m16n8k64,
                        row = row,
                        col = col,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vec_size = scale_vec_size,
                        f32 = f32,
                        e2m1 = e2m1,
                        e2m12 = e2m12,
                        f322 = f322,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,
                        scale_a_data = scale_a_data,
                        byte_id_a = byte_id_a,
                        scale_b_data = scale_b_data,
                        byte_id_b = byte_id_b,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::mma::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".kind::mxf4nvf4"), |_, _span| {
                Kind::KindMxf4nvf4
            }))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".scale_vec::2X"), |_, _span| {
                    ScaleVecSize::ScaleVec2x
                }),
                map(string_p(".scale_vec::4X"), |_, _span| {
                    ScaleVecSize::ScaleVec4x
                })
            )
        }
    }

    impl PtxParser for Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".ue8m0"), |_, _span| Stype::Ue8m0),
                map(string_p(".ue4m3"), |_, _span| Stype::Ue4m3)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k64"),
                    string_p(".row"),
                    string_p(".col"),
                    Kind::parse(),
                    string_p(".block_scale"),
                    ScaleVecSize::parse(),
                    string_p(".f32"),
                    string_p(".e2m1"),
                    string_p(".e2m1"),
                    string_p(".f32"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k64,
                    row,
                    col,
                    kind,
                    block_scale,
                    scale_vec_size,
                    f32,
                    e2m1,
                    e2m12,
                    f322,
                    stype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                    scale_a_data,
                    _,
                    byte_id_a,
                    _,
                    scale_b_data,
                    _,
                    byte_id_b,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 {
                        sync = sync,
                        aligned = aligned,
                        m16n8k64 = m16n8k64,
                        row = row,
                        col = col,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vec_size = scale_vec_size,
                        f32 = f32,
                        e2m1 = e2m1,
                        e2m12 = e2m12,
                        f322 = f322,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,
                        scale_a_data = scale_a_data,
                        byte_id_a = byte_id_a,
                        scale_b_data = scale_b_data,
                        byte_id_b = byte_id_b,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::mma::section_4::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for F8f6f4type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".e4m3"), |_, _span| F8f6f4type::E4m3),
                map(string_p(".e5m2"), |_, _span| F8f6f4type::E5m2),
                map(string_p(".e3m2"), |_, _span| F8f6f4type::E3m2),
                map(string_p(".e2m3"), |_, _span| F8f6f4type::E2m3),
                map(string_p(".e2m1"), |_, _span| F8f6f4type::E2m1)
            )
        }
    }

    impl PtxParser for Kind {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".kind::mxf8f6f4"), |_, _span| {
                Kind::KindMxf8f6f4
            }))
        }
    }

    impl PtxParser for ScaleVecSize {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".scale_vec::1X"), |_, _span| {
                ScaleVecSize::ScaleVec1x
            }))
        }
    }

    impl PtxParser for Stype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".ue8m0"), |_, _span| Stype::Ue8m0))
        }
    }

    impl PtxParser
        for MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype
    {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".m16n8k32"),
                    string_p(".row"),
                    string_p(".col"),
                    Kind::parse(),
                    string_p(".block_scale"),
                    optional(ScaleVecSize::parse()),
                    string_p(".f32"),
                    F8f6f4type::parse(),
                    F8f6f4type::parse(),
                    string_p(".f32"),
                    Stype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    VectorOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    m16n8k32,
                    row,
                    col,
                    kind,
                    block_scale,
                    scale_vec_size,
                    f32,
                    f8f6f4type,
                    f8f6f4type1,
                    f322,
                    stype,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                    scale_a_data,
                    _,
                    byte_id_a,
                    _,
                    scale_b_data,
                    _,
                    byte_id_b,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype {
                        sync = sync,
                        aligned = aligned,
                        m16n8k32 = m16n8k32,
                        row = row,
                        col = col,
                        kind = kind,
                        block_scale = block_scale,
                        scale_vec_size = scale_vec_size,
                        f32 = f32,
                        f8f6f4type = f8f6f4type,
                        f8f6f4type1 = f8f6f4type1,
                        f322 = f322,
                        stype = stype,
                        d = d,
                        a = a,
                        b = b,
                        c = c,
                        scale_a_data = scale_a_data,
                        byte_id_a = byte_id_a,
                        scale_b_data = scale_b_data,
                        byte_id_b = byte_id_b,

                    })
                },
            )
        }
    }
}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::mma::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8k16"), |_, _span| Shape::M16n8k16),
                map(string_p(".m16n8k4"), |_, _span| Shape::M16n8k4),
                map(string_p(".m16n8k8"), |_, _span| Shape::M16n8k8),
                map(string_p(".m8n84"), |_, _span| Shape::M8n84)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColF64F64F64F64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    string_p(".row"),
                    string_p(".col"),
                    string_p(".f64"),
                    string_p(".f64"),
                    string_p(".f64"),
                    string_p(".f64"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    shape,
                    row,
                    col,
                    f64,
                    f642,
                    f644,
                    f646,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedShapeRowColF64F64F64F64 {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        row = row,
                        col = col,
                        f64 = f64,
                        f642 = f642,
                        f644 = f644,
                        f646 = f646,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_6 {
    use super::*;
    use crate::r#type::instruction::mma::section_6::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u8"), |_, _span| Atype::U8),
                map(string_p(".s8"), |_, _span| Atype::S8)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u8"), |_, _span| Btype::U8),
                map(string_p(".s8"), |_, _span| Btype::S8)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8k16"), |_, _span| Shape::M16n8k16),
                map(string_p(".m16n8k32"), |_, _span| Shape::M16n8k32),
                map(string_p(".m8n8k16"), |_, _span| Shape::M8n8k16)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    string_p(".row"),
                    string_p(".col"),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".s32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    shape,
                    row,
                    col,
                    satfinite,
                    s32,
                    atype,
                    btype,
                    s322,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        row = row,
                        col = col,
                        satfinite = satfinite,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        s322 = s322,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_7 {
    use super::*;
    use crate::r#type::instruction::mma::section_7::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u4"), |_, _span| Atype::U4),
                map(string_p(".s4"), |_, _span| Atype::S4)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".u4"), |_, _span| Btype::U4),
                map(string_p(".s4"), |_, _span| Btype::S4)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8k32"), |_, _span| Shape::M16n8k32),
                map(string_p(".m16n8k64"), |_, _span| Shape::M16n8k64),
                map(string_p(".m8n8k32"), |_, _span| Shape::M8n8k32)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    string_p(".row"),
                    string_p(".col"),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".s32"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    shape,
                    row,
                    col,
                    satfinite,
                    s32,
                    atype,
                    btype,
                    s322,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        row = row,
                        col = col,
                        satfinite = satfinite,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        s322 = s322,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}

pub mod section_8 {
    use super::*;
    use crate::r#type::instruction::mma::section_8::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Bitop {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".xor"), |_, _span| Bitop::Xor),
                map(string_p(".and"), |_, _span| Bitop::And)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8k128"), |_, _span| Shape::M16n8k128),
                map(string_p(".m16n8k256"), |_, _span| Shape::M16n8k256),
                map(string_p(".m8n8k128"), |_, _span| Shape::M8n8k128)
            )
        }
    }

    impl PtxParser for MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    string_p(".row"),
                    string_p(".col"),
                    string_p(".s32"),
                    string_p(".b1"),
                    string_p(".b1"),
                    string_p(".s32"),
                    Bitop::parse(),
                    string_p(".popc"),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    sync,
                    aligned,
                    shape,
                    row,
                    col,
                    s32,
                    b1,
                    b12,
                    s322,
                    bitop,
                    popc,
                    d,
                    _,
                    a,
                    _,
                    b,
                    _,
                    c,
                    _,
                ),
                 span| {
                    ok!(MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        row = row,
                        col = col,
                        s32 = s32,
                        b1 = b1,
                        b12 = b12,
                        s322 = s322,
                        bitop = bitop,
                        popc = popc,
                        d = d,
                        a = a,
                        b = b,
                        c = c,

                    })
                },
            )
        }
    }
}
