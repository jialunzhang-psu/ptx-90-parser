//! Original PTX specification:
//!
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32, .m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f64};

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
    use crate::r#type::instruction::wmma_store::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".f16"), |_, _span| Type::F16),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".s32"), |_, _span| Type::S32)
            )
        }
    }

    impl PtxParser for WmmaStoreDSyncAlignedLayoutShapeSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".store"),
                    string_p(".d"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, store, d, sync, aligned, layout, shape, ss, type_, p, _, r, stride, _),
                 span| {
                    ok!(WmmaStoreDSyncAlignedLayoutShapeSsType {
                        store = store,
                        d = d,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        type_ = type_,
                        p = p,
                        r = r,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
                map(string_p(".m8n8k128"), |_, _span| Shape::M8n8k128),
                map(string_p(".m8n8k32"), |_, _span| Shape::M8n8k32)
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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".s32"), |_, _span| Type::S32))
        }
    }

    impl PtxParser for WmmaStoreDSyncAlignedLayoutShapeSsType1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".store"),
                    string_p(".d"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, store, d, sync, aligned, layout, shape, ss, type_, p, _, r, stride, _),
                 span| {
                    ok!(WmmaStoreDSyncAlignedLayoutShapeSsType1 {
                        store = store,
                        d = d,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        type_ = type_,
                        p = p,
                        r = r,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f32"), |_, _span| Type::F32))
        }
    }

    impl PtxParser for WmmaStoreDSyncAlignedLayoutShapeSsType2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".store"),
                    string_p(".d"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, store, d, sync, aligned, layout, shape, ss, type_, p, _, r, stride, _),
                 span| {
                    ok!(WmmaStoreDSyncAlignedLayoutShapeSsType2 {
                        store = store,
                        d = d,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        type_ = type_,
                        p = p,
                        r = r,
                        stride = stride,

                    })
                },
            )
        }
    }
}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_3::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".f64"), |_, _span| Type::F64))
        }
    }

    impl PtxParser for WmmaStoreDSyncAlignedLayoutShapeSsType3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("wmma"),
                    string_p(".store"),
                    string_p(".d"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Layout::parse(),
                    Shape::parse(),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    map(
                        optional(seq_n!(comma_p(), GeneralOperand::parse())),
                        |value, _| value.map(|(_, operand)| operand)
                    ),
                    semicolon_p()
                ),
                |(_, store, d, sync, aligned, layout, shape, ss, type_, p, _, r, stride, _),
                 span| {
                    ok!(WmmaStoreDSyncAlignedLayoutShapeSsType3 {
                        store = store,
                        d = d,
                        sync = sync,
                        aligned = aligned,
                        layout = layout,
                        shape = shape,
                        ss = ss,
                        type_ = type_,
                        p = p,
                        r = r,
                        stride = stride,

                    })
                },
            )
        }
    }
}
