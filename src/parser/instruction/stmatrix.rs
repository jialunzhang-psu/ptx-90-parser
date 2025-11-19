//! Original PTX specification:
//!
//! stmatrix.sync.aligned.shape.num{.trans}{.ss}.type [p], r;
//! .shape  = {.m8n8, .m16n8};
//! .num    = {.x1, .x2, .x4};
//! .ss     = {.shared, .shared::cta};
//! .type   = {.b16, .b8};

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
    use crate::r#type::instruction::stmatrix::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Num {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".x1"), |_, _span| Num::X1),
                map(string_p(".x2"), |_, _span| Num::X2),
                map(string_p(".x4"), |_, _span| Num::X4)
            )
        }
    }

    impl PtxParser for Shape {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".m16n8"), |_, _span| Shape::M16n8),
                map(string_p(".m8n8"), |_, _span| Shape::M8n8)
            )
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".shared"), |_, _span| Ss::Shared)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Type::B16),
                map(string_p(".b8"), |_, _span| Type::B8)
            )
        }
    }

    impl PtxParser for StmatrixSyncAlignedShapeNumTransSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("stmatrix"),
                    string_p(".sync"),
                    string_p(".aligned"),
                    Shape::parse(),
                    Num::parse(),
                    map(optional(string_p(".trans")), |value, _| value.is_some()),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, sync, aligned, shape, num, trans, ss, type_, p, _, r, _), span| {
                    ok!(StmatrixSyncAlignedShapeNumTransSsType {
                        sync = sync,
                        aligned = aligned,
                        shape = shape,
                        num = num,
                        trans = trans,
                        ss = ss,
                        type_ = type_,
                        p = p,
                        r = r,

                    })
                },
            )
        }
    }
}
