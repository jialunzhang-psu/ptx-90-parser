//! Original PTX specification:
//!
//! // Atomic operation with scalar type:
//! atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.op.type d, [a], b, c;
//! atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;
//! atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;
//! atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b {, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};
//! .space =              { .global, .shared, .shared::cta, .shared::cluster};
//! .sem =                { .relaxed, .acquire, .release, .acq_rel };
//! .scope =              { .cta, .cluster, .gpu, .sys };
//! .op =                 { .and, .or, .xor, .cas, .exch, .add, .inc, .dec, .min, .max };
//! .level::cache_hint =  { .L2::cache_hint };
//! .type =               { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
//! -------------------------------------------------------------
//! // Atomic operation with vector type:
//! atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32                  d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type  d, [a], b{, cache-policy};
//! atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type     d, [a], b{, cache-policy};
//! .sem =               { .relaxed, .acquire, .release, .acq_rel };
//! .scope =             { .cta, .cluster, .gpu, .sys };
//! .op =                { .add, .min, .max };
//! .half_word_type =    { .f16, .bf16 };
//! .packed_type =       { .f16x2, .bf16x2 };
//! .vec_16_bit =        { .v2, .v4, .v8 };
//! .vec_32_bit =        { .v2, .v4 };
//! .level::cache_hint = { .L2::cache_hint };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::atom::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for LevelCacheHint {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2CacheHint
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::cache_hint").is_ok() {
                    return Ok(LevelCacheHint::L2CacheHint);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Exch
            {
                let saved_pos = stream.position();
                if stream.expect_string(".exch").is_ok() {
                    return Ok(Op::Exch);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Op::And);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Op::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Cas
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cas").is_ok() {
                    return Ok(Op::Cas);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Inc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".inc").is_ok() {
                    return Ok(Op::Inc);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Dec
            {
                let saved_pos = stream.position();
                if stream.expect_string(".dec").is_ok() {
                    return Ok(Op::Dec);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Op::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".exch", ".and", ".xor", ".cas", ".add", ".inc", ".dec", ".min", ".max", ".or"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Sem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Sem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Acquire
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acquire").is_ok() {
                    return Ok(Sem::Acquire);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Sem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AcqRel
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acq_rel").is_ok() {
                    return Ok(Sem::AcqRel);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".acquire", ".release", ".acq_rel"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Space {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cluster").is_ok() {
                    return Ok(Space::SharedCluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Space::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Space::Global);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Shared
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared").is_ok() {
                    return Ok(Space::Shared);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cluster", ".shared::cta", ".global", ".shared"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try B32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b32").is_ok() {
                    return Ok(Type::B32);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try B64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".b64").is_ok() {
                    return Ok(Type::B64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u32").is_ok() {
                    return Ok(Type::U32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s32").is_ok() {
                    return Ok(Type::S32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try S64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".s64").is_ok() {
                    return Ok(Type::S64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F32
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f32").is_ok() {
                    return Ok(Type::F32);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try F64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f64").is_ok() {
                    return Ok(Type::F64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".b64", ".u32", ".u64", ".s32", ".s64", ".f32", ".f64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AtomSemScopeSpaceOpLevelCacheHintType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceOpLevelCacheHintType {
                sem,
                scope,
                space,
                op,
                level_cache_hint,
                type_,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceOpType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            let type_ = Type::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceOpType {
                sem,
                scope,
                space,
                op,
                type_,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceCasB16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".cas")?;
            let cas = ();
            stream.expect_complete()?;
            stream.expect_string(".b16")?;
            let b16 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceCasB16 {
                sem,
                scope,
                space,
                cas,
                b16,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceCasB128 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".cas")?;
            let cas = ();
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let c = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceCasB128 {
                sem,
                scope,
                space,
                cas,
                b128,
                d,
                a,
                b,
                c,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceExchLevelCacheHintB128 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".exch")?;
            let exch = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".b128")?;
            let b128 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceExchLevelCacheHintB128 {
                sem,
                scope,
                space,
                exch,
                level_cache_hint,
                b128,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".f16")?;
            let f16 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
                sem,
                scope,
                space,
                add,
                noftz,
                level_cache_hint,
                f16,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".f16x2")?;
            let f16x2 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
                sem,
                scope,
                space,
                add,
                noftz,
                level_cache_hint,
                f16x2,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".bf16")?;
            let bf16 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
                sem,
                scope,
                space,
                add,
                noftz,
                level_cache_hint,
                bf16,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let space = match Space::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_string(".bf16x2")?;
            let bf16x2 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
                sem,
                scope,
                space,
                add,
                noftz,
                level_cache_hint,
                bf16x2,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::atom::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for HalfWordType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(HalfWordType::Bf16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(HalfWordType::F16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16", ".f16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LevelCacheHint {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try L2CacheHint
            {
                let saved_pos = stream.position();
                if stream.expect_string(".L2::cache_hint").is_ok() {
                    return Ok(LevelCacheHint::L2CacheHint);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Op {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Op::Add);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Op::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Op::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".add", ".min", ".max"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for PackedType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Bf16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16x2").is_ok() {
                    return Ok(PackedType::Bf16x2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try F16x2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16x2").is_ok() {
                    return Ok(PackedType::F16x2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bf16x2", ".f16x2"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Scope {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Cluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cluster").is_ok() {
                    return Ok(Scope::Cluster);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Cta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta").is_ok() {
                    return Ok(Scope::Cta);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Gpu
            {
                let saved_pos = stream.position();
                if stream.expect_string(".gpu").is_ok() {
                    return Ok(Scope::Gpu);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Sys
            {
                let saved_pos = stream.position();
                if stream.expect_string(".sys").is_ok() {
                    return Ok(Scope::Sys);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cluster", ".cta", ".gpu", ".sys"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Sem {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Relaxed
            {
                let saved_pos = stream.position();
                if stream.expect_string(".relaxed").is_ok() {
                    return Ok(Sem::Relaxed);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Acquire
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acquire").is_ok() {
                    return Ok(Sem::Acquire);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Release
            {
                let saved_pos = stream.position();
                if stream.expect_string(".release").is_ok() {
                    return Ok(Sem::Release);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try AcqRel
            {
                let saved_pos = stream.position();
                if stream.expect_string(".acq_rel").is_ok() {
                    return Ok(Sem::AcqRel);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".relaxed", ".acquire", ".release", ".acq_rel"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vec16Bit {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try V2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v2").is_ok() {
                    return Ok(Vec16Bit::V2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try V4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v4").is_ok() {
                    return Ok(Vec16Bit::V4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try V8
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v8").is_ok() {
                    return Ok(Vec16Bit::V8);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".v2", ".v4", ".v8"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Vec32Bit {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try V2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v2").is_ok() {
                    return Ok(Vec32Bit::V2);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try V4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".v4").is_ok() {
                    return Ok(Vec32Bit::V4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".v2", ".v4"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let vec_32_bit = Vec32Bit::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".f32")?;
            let f32 = ();
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
                sem,
                scope,
                global,
                add,
                level_cache_hint,
                vec_32_bit,
                f32,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let vec_16_bit = Vec16Bit::parse(stream)?;
            stream.expect_complete()?;
            let half_word_type = HalfWordType::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
                sem,
                scope,
                global,
                op,
                noftz,
                level_cache_hint,
                vec_16_bit,
                half_word_type,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


    impl PtxParser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("atom")?;
            let saved_pos = stream.position();
            let sem = match Sem::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let scope = match Scope::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let global = stream.expect_string(".global").is_ok();
            if !global {
                stream.set_position(saved_pos);
            }
            stream.expect_complete()?;
            let op = Op::parse(stream)?;
            stream.expect_complete()?;
            stream.expect_string(".noftz")?;
            let noftz = ();
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let vec_32_bit = Vec32Bit::parse(stream)?;
            stream.expect_complete()?;
            let packed_type = PackedType::parse(stream)?;
            stream.expect_complete()?;
            let d = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let a = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let b = GeneralOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Semicolon)?;
            Ok(AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
                sem,
                scope,
                global,
                op,
                noftz,
                level_cache_hint,
                vec_32_bit,
                packed_type,
                d,
                a,
                b,
                cache_policy,
            })
        }
    }


}

