//! Original PTX specification:
//!
//! st.async{.sem}{.scope}{.ss}{.completion_mechanism}{.vec}.type [a], b, [mbar];
//! .sem  =                 { .weak };
//! .scope =                { .cluster };
//! .ss   =                 { .shared::cluster };
//! .type =                 { .b32, .b64,
//! .u32, .u64,
//! .s32, .s64,
//! .f32, .f64 };
//! .vec  =                 { .v2, .v4 };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! ---------------------------------------------------------
//! st.async{.mmio}.sem.scope{.ss}.type [a], b;
//! .sem =                  { .release };
//! .scope =                { .gpu, .sys };
//! .ss =                   { .global };
//! .type =                 { .b8, .b16, .b32, .b64,
//! .u8, .u16, .u32, .u64,
//! .s8, .s16, .s32, .s64,
//! .f32, .f64 };

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
    use crate::r#type::instruction::st_async::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(
                string_p(".mbarrier::complete_tx::bytes"),
                |_, _span| CompletionMechanism::MbarrierCompleteTxBytes
            ))
        }
    }

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".cluster"), |_, _span| Scope::Cluster))
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".weak"), |_, _span| Sem::Weak))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".shared::cluster"), |_, _span| {
                Ss::SharedCluster
            }))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64)
            )
        }
    }

    impl PtxParser for Vec {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".v2"), |_, _span| Vec::V2),
                map(string_p(".v4"), |_, _span| Vec::V4)
            )
        }
    }

    impl PtxParser for StAsyncSemScopeSsCompletionMechanismVecType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".async"),
                    optional(Sem::parse()),
                    optional(Scope::parse()),
                    optional(Ss::parse()),
                    optional(CompletionMechanism::parse()),
                    optional(Vec::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    comma_p(),
                    AddressOperand::parse(),
                    semicolon_p()
                ),
                |(
                    _,
                    async_,
                    sem,
                    scope,
                    ss,
                    completion_mechanism,
                    vec,
                    type_,
                    a,
                    _,
                    b,
                    _,
                    mbar,
                    _,
                ),
                 span| {
                    ok!(StAsyncSemScopeSsCompletionMechanismVecType {
                        async_ = async_,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        completion_mechanism = completion_mechanism,
                        vec = vec,
                        type_ = type_,
                        a = a,
                        b = b,
                        mbar = mbar,

                    })
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::st_async::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Scope {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".gpu"), |_, _span| Scope::Gpu),
                map(string_p(".sys"), |_, _span| Scope::Sys)
            )
        }
    }

    impl PtxParser for Sem {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".release"), |_, _span| Sem::Release))
        }
    }

    impl PtxParser for Ss {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(map(string_p(".global"), |_, _span| Ss::Global))
        }
    }

    impl PtxParser for Type {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            alt!(
                map(string_p(".b16"), |_, _span| Type::B16),
                map(string_p(".b32"), |_, _span| Type::B32),
                map(string_p(".b64"), |_, _span| Type::B64),
                map(string_p(".u16"), |_, _span| Type::U16),
                map(string_p(".u32"), |_, _span| Type::U32),
                map(string_p(".u64"), |_, _span| Type::U64),
                map(string_p(".s16"), |_, _span| Type::S16),
                map(string_p(".s32"), |_, _span| Type::S32),
                map(string_p(".s64"), |_, _span| Type::S64),
                map(string_p(".f32"), |_, _span| Type::F32),
                map(string_p(".f64"), |_, _span| Type::F64),
                map(string_p(".b8"), |_, _span| Type::B8),
                map(string_p(".u8"), |_, _span| Type::U8),
                map(string_p(".s8"), |_, _span| Type::S8)
            )
        }
    }

    impl PtxParser for StAsyncMmioSemScopeSsType {
        fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {
            try_map(
                seq_n!(
                    string_p("st"),
                    string_p(".async"),
                    map(optional(string_p(".mmio")), |value, _| value.is_some()),
                    Sem::parse(),
                    Scope::parse(),
                    optional(Ss::parse()),
                    Type::parse(),
                    AddressOperand::parse(),
                    comma_p(),
                    GeneralOperand::parse(),
                    semicolon_p()
                ),
                |(_, async_, mmio, sem, scope, ss, type_, a, _, b, _), span| {
                    ok!(StAsyncMmioSemScopeSsType {
                        async_ = async_,
                        mmio = mmio,
                        sem = sem,
                        scope = scope,
                        ss = ss,
                        type_ = type_,
                        a = a,
                        b = b,

                    })
                },
            )
        }
    }
}
