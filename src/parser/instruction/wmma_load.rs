//! Original PTX specification:
//!
//! // Floating point format .f16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f16, .s8, .u8};
//! .btype  = {.f16, .s8, .u8};
//! .ctype  = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! // Alternate floating point format .bf16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.bf16 };
//! .btype  = {.bf16 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Alternate floating point format .tf32 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.tf32 };
//! .btype  = {.tf32 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Double precision Floating point .f64 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f64 };
//! .btype  = {.f64 };
//! .ctype  = {.f64 };
//! ----------------------------------------------------------------
//! // Sub-byte loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! .ctype  = {.s32};
//! ----------------------------------------------------------------
//! // Single-bit loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .ctype  = {.s32};

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
    use crate::r#type::instruction::wmma_load::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Atype::F16),
                map(string_p(".s8"), |_, _span| Atype::S8),
                map(string_p(".u8"), |_, _span| Atype::U8)
            )
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Btype::F16),
                map(string_p(".s8"), |_, _span| Btype::S8),
                map(string_p(".u8"), |_, _span| Btype::U8)
            )
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Ctype::F16),
                map(string_p(".f32"), |_, _span| Ctype::F32),
                map(string_p(".s32"), |_, _span| Ctype::S32)
            )
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
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

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, layout, shape, ss, atype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadASyncAlignedLayoutShapeSsAtype {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, layout, shape, ss, btype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadBSyncAlignedLayoutShapeSsBtype {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".bf16"), |_, _span| Atype::Bf16))
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".bf16"), |_, _span| Btype::Bf16))
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f32"), |_, _span| Ctype::F32))
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
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

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, layout, shape, ss, atype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, layout, shape, ss, btype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tf32"), |_, _span| Atype::Tf32))
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tf32"), |_, _span| Btype::Tf32))
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f32"), |_, _span| Ctype::F32))
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m16n16k8"), |_, _span| Shape::M16n16k8))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, layout, shape, ss, atype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, layout, shape, ss, btype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Atype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f64"), |_, _span| Atype::F64))
        }
    }

    impl PtxParser for Btype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f64"), |_, _span| Btype::F64))
        }
    }

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f64"), |_, _span| Ctype::F64))
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k4"), |_, _span| Shape::M8n8k4))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, layout, shape, ss, atype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, layout, shape, ss, btype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_4::*;

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

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".s32"), |_, _span| Ctype::S32))
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k32"), |_, _span| Shape::M8n8k32))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedRowShapeSsAtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".row"),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, row, shape, ss, atype, r, _, p, stride, _), span| {
                    ok!(WmmaLoadASyncAlignedRowShapeSsAtype {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        row = row,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedColShapeSsBtype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".col"),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, col, shape, ss, btype, r, _, p, stride, _), span| {
                    ok!(WmmaLoadBSyncAlignedColShapeSsBtype {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        col = col,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_5::*;

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

    impl PtxParser for Ctype {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".s32"), |_, _span| Ctype::S32))
        }
    }

    impl PtxParser for Layout {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".row"), |_, _span| Layout::Row),
                map(string_p(".col"), |_, _span| Layout::Col)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".m8n8k128"), |_, _span| Shape::M8n8k128))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for WmmaLoadASyncAlignedRowShapeSsAtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".a"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".row"),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Atype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, a, sync, aligned, row, shape, ss, atype, r, _, p, stride, _), span| {
                    ok!(WmmaLoadASyncAlignedRowShapeSsAtype1 {
                        load = load,
                        a = a,
                        sync = sync,
                        aligned = aligned,
                        row = row,
                        shape = shape,
                        ss = ss,
                        atype = atype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadBSyncAlignedColShapeSsBtype1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".b"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    string_p(".col"),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Btype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, b, sync, aligned, col, shape, ss, btype, r, _, p, stride, _), span| {
                    ok!(WmmaLoadBSyncAlignedColShapeSsBtype1 {
                        load = load,
                        b = b,
                        sync = sync,
                        aligned = aligned,
                        col = col,
                        shape = shape,
                        ss = ss,
                        btype = btype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }

    impl PtxParser for WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".load"),
                    string_p(".c"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Ctype::parse(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, load, c, sync, aligned, layout, shape, ss, ctype, r, _, p, stride, _),
                 span| {
                    ok!(WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
                        load = load,
                        c = c,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        ctype = ctype,
                        r = r,
                        p = p,
                        stride = stride,

                    })
                },
            )
        }
    }
}
