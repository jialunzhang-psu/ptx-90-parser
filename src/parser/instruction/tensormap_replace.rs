//! Original PTX specification:
//!
//! tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
//! tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
//! tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;
//! .mode    = { .tile };
//! .field1  = { .global_address, .rank };
//! .field2  = { .box_dim, .global_dim, .global_stride, .element_stride  };
//! .field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode };
//! .ss      = { .global, .shared::cta };
//! .type    = { .b32, .b64 };

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
    use crate::r#type::instruction::tensormap_replace::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Field1 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".global_address"), |_, _span| {
                    Field1::GlobalAddress
                }),
                map(string_p(".rank"), |_, _span| Field1::Rank)
            )
        }
    }

    impl PtxParser for Field2 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".element_stride"), |_, _span| {
                    Field2::ElementStride
                }),
                map(string_p(".global_stride"), |_, _span| Field2::GlobalStride),
                map(string_p(".global_dim"), |_, _span| Field2::GlobalDim),
                map(string_p(".box_dim"), |_, _span| Field2::BoxDim)
            )
        }
    }

    impl PtxParser for Field3 {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".interleave_layout"), |_, _span| {
                    Field3::InterleaveLayout
                }),
                map(string_p(".swizzle_atomicity"), |_, _span| {
                    Field3::SwizzleAtomicity
                }),
                map(string_p(".swizzle_mode"), |_, _span| Field3::SwizzleMode),
                map(string_p(".fill_mode"), |_, _span| Field3::FillMode),
                map(string_p(".elemtype"), |_, _span| Field3::Elemtype)
            )
        }
    }

    impl PtxParser for Mode {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".tile"), |_, _span| Mode::Tile))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".shared::cta"), |_, _span| Ss::SharedCta),
                map(string_p(".global"), |_, _span| Ss::Global)
            )
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64)
            )
        }
    }

    impl PtxParser for TensormapReplaceModeField1SsB1024Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tensormap"),
                    string_p(".replace"),
                    Mode::parse(),
                    Field1::parse(),
                    optional(Ss::parse()),
                    string_p(".b1024"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, replace, mode, field1, ss, b1024, type_, addr, _, new_val, _), span| {
                    ok!(TensormapReplaceModeField1SsB1024Type {
                        replace = replace,
                        mode = mode,
                        field1 = field1,
                        ss = ss,
                        b1024 = b1024,
                        type_ = type_,
                        addr = addr,
                        new_val = new_val,

                    })
                },
            )
        }
    }

    impl PtxParser for TensormapReplaceModeField2SsB1024Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tensormap"),
                    string_p(".replace"),
                    Mode::parse(),
                    Field2::parse(),
                    optional(Ss::parse()),
                    string_p(".b1024"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, replace, mode, field2, ss, b1024, type_, addr, _, ord, _, new_val, _),
                 span| {
                    ok!(TensormapReplaceModeField2SsB1024Type {
                        replace = replace,
                        mode = mode,
                        field2 = field2,
                        ss = ss,
                        b1024 = b1024,
                        type_ = type_,
                        addr = addr,
                        ord = ord,
                        new_val = new_val,

                    })
                },
            )
        }
    }

    impl PtxParser for TensormapReplaceModeField3SsB1024Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("tensormap"),
                    string_p(".replace"),
                    Mode::parse(),
                    Field3::parse(),
                    optional(Ss::parse()),
                    string_p(".b1024"),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, replace, mode, field3, ss, b1024, type_, addr, _, new_val, _), span| {
                    ok!(TensormapReplaceModeField3SsB1024Type {
                        replace = replace,
                        mode = mode,
                        field3 = field3,
                        ss = ss,
                        b1024 = b1024,
                        type_ = type_,
                        addr = addr,
                        new_val = new_val,

                    })
                },
            )
        }
    }
}
