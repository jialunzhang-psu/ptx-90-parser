//! Original PTX specification:
//!
//! // Floating point (.f16 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.dtype.ctype d, a, b, c;
//! ----------------------------------------------------------------
//! // Integer (.u8/.s8 multiplicands) wmma.mma
//! wmma.mma.sync.aligned.alayout.blayout.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape  =  {.m16n16k16, .m8n32k16, .m32n8k16};
//! .dtype   = {.f16, .f32};
//! .atype   = {.s8, .u8};
//! .btype   = {.s8, .u8};
//! .ctype   = {.f16, .f32};
//! ----------------------------------------------------------------
//! // Floating point format .bf16 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .atype   = {.bf16 };
//! .btype   = {.bf16};
//! ----------------------------------------------------------------
//! // Floating point format .tf32 wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape.f32.atype.btype.f32 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m16n16k8 };
//! .atype   = {.tf32 };
//! .btype   = {.tf32};
//! ----------------------------------------------------------------
//! // Floating point Double precision wmma.mma:
//! wmma.mma.sync.aligned.alayout.blayout.shape{.rnd}.f64.f64.f64.f64 d, a, b, c;
//! .alayout = {.row, .col};
//! .blayout = {.row, .col};
//! .shape   = {.m8n8k4 };
//! .rnd = { .rn, .rz, .rm, .rp };
//! ----------------------------------------------------------------
//! // Sub-byte (.u4/.s4 multiplicands) wmma.mma:
//! wmma.mma.sync.aligned.row.col.shape.s32.atype.btype.s32{.satfinite} d, a, b, c;
//! .shape  = {.m8n8k32};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! ----------------------------------------------------------------
//! // Single-bit (.b1 multiplicands) wmma.mma:
//! wmma.mma.op.popc.sync.aligned.row.col.shape.s32.atype.btype.s32 d, a, b, c;
//! .shape  = {.m8n8k128};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .op     = {.xor, .and};

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
    use crate::r#type::instruction::wmma_mma::section_0::*;

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".alayout"),
                    string_p(".blayout"),
                    string_p(".shape"),
                    string_p(".dtype"),
                    string_p(".ctype"),
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
                    mma,
                    sync,
                    aligned,
                    alayout,
                    blayout,
                    shape,
                    dtype,
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
                    ok!(WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        alayout = alayout,
                        blayout = blayout,
                        shape = shape,
                        dtype = dtype,
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
    use crate::r#type::instruction::wmma_mma::section_1::*;

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

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s8"), |_, _span| Atype::S8),
                map(string_p(".u8"), |_, _span| Atype::U8)
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

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s8"), |_, _span| Btype::S8),
                map(string_p(".u8"), |_, _span| Btype::U8)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n16k16"), |_, _span| Shape::M16n16k16),
                map(string_p(".m8n32k16"), |_, _span| Shape::M8n32k16),
                map(string_p(".m32n8k16"), |_, _span| Shape::M32n8k16)
            )
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Alayout::parse(),
                    Blayout::parse(),
                    Shape::parse(),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".s32"),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
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
                    mma,
                    sync,
                    aligned,
                    alayout,
                    blayout,
                    shape,
                    s32,
                    atype,
                    btype,
                    s322,
                    satfinite,
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
                    ok!(WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        alayout = alayout,
                        blayout = blayout,
                        shape = shape,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        s322 = s322,
                        satfinite = satfinite,
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
    use crate::r#type::instruction::wmma_mma::section_2::*;

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

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".bf16"), |_, _span| Atype::Bf16))
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

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".bf16"), |_, _span| Btype::Bf16))
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n16k16"), |_, _span| Shape::M16n16k16),
                map(string_p(".m8n32k16"), |_, _span| Shape::M8n32k16),
                map(string_p(".m32n8k16"), |_, _span| Shape::M32n8k16)
            )
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Alayout::parse(),
                    Blayout::parse(),
                    Shape::parse(),
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
                    mma,
                    sync,
                    aligned,
                    alayout,
                    blayout,
                    shape,
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
                    ok!(WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        alayout = alayout,
                        blayout = blayout,
                        shape = shape,
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
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_3::*;

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

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tf32"), |_, _span| Atype::Tf32))
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

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tf32"), |_, _span| Btype::Tf32))
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m16n16k8"), |_, _span| Shape::M16n16k8))
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Alayout::parse(),
                    Blayout::parse(),
                    Shape::parse(),
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
                    mma,
                    sync,
                    aligned,
                    alayout,
                    blayout,
                    shape,
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
                    ok!(WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        alayout = alayout,
                        blayout = blayout,
                        shape = shape,
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
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_4::*;

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

    impl PtxParser for Rnd {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".rn"), |_, _span| Rnd::Rn),
                map(string_p(".rz"), |_, _span| Rnd::Rz),
                map(string_p(".rm"), |_, _span| Rnd::Rm),
                map(string_p(".rp"), |_, _span| Rnd::Rp)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k4"), |_, _span| Shape::M8n8k4))
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Alayout::parse(),
                    Blayout::parse(),
                    Shape::parse(),
                    optional(Rnd::parse()),
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
                    mma,
                    sync,
                    aligned,
                    alayout,
                    blayout,
                    shape,
                    rnd,
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
                    ok!(WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        alayout = alayout,
                        blayout = blayout,
                        shape = shape,
                        rnd = rnd,
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

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_mma::section_5::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s4"), |_, _span| Atype::S4),
                map(string_p(".u4"), |_, _span| Atype::U4)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".s4"), |_, _span| Btype::S4),
                map(string_p(".u4"), |_, _span| Btype::U4)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k32"), |_, _span| Shape::M8n8k32))
        }
    }

    impl PtxParser for WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".row"),
                    string_p(".col"),
                    Shape::parse(),
                    string_p(".s32"),
                    Atype::parse(),
                    Btype::parse(),
                    string_p(".s32"),
                    map(optional(string_p(".satfinite")), |value, _| value.is_some()),
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
                    mma,
                    sync,
                    aligned,
                    row,
                    col,
                    shape,
                    s32,
                    atype,
                    btype,
                    s322,
                    satfinite,
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
                    ok!(WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite {
                        mma = mma,
                        sync = sync,
                        aligned = aligned,
                        row = row,
                        col = col,
                        shape = shape,
                        s32 = s32,
                        atype = atype,
                        btype = btype,
                        s322 = s322,
                        satfinite = satfinite,
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
    use crate::r#type::instruction::wmma_mma::section_6::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".b1"), |_, _span| Atype::B1))
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".b1"), |_, _span| Btype::B1))
        }
    }

    impl PtxParser for Op {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".xor"), |_, _span| Op::Xor),
                map(string_p(".and"), |_, _span| Op::And)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k128"), |_, _span| Shape::M8n8k128))
        }
    }

    impl PtxParser for WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".mma"),
                    Op::parse(),
                    string_p(".popc"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".row"),
                    string_p(".col"),
                    Shape::parse(),
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
                    mma,
                    op,
                    popc,
                    sync,
                    aligned,
                    row,
                    col,
                    shape,
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
                    ok!(WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 {
                        mma = mma,
                        op = op,
                        popc = popc,
                        sync = sync,
                        aligned = aligned,
                        row = row,
                        col = col,
                        shape = shape,
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
