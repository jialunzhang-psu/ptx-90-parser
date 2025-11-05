//! Original PTX specification:
//!
//! cp.reduce.async.bulk.dst.src.completion_mechanism.redOp.type [dstMem], [srcMem], size, [mbar];
//! .dst =                  { .shared::cluster };
//! .src =                  { .shared::cta };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .b32, .u32, .s32, .b64, .u64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.redOp.type [dstMem], [srcMem], size{, cache-policy};
//! .dst =                  { .global      };
//! .src =                  { .shared::cta };
//! ----------------------------------------------------------------
//! .completion_mechanism = { .bulk_group };
//! .level::cache_hint    = { .L2::cache_hint };
//! .redOp=                 { .and, .or, .xor, .add, .inc, .dec, .min, .max };
//! .type =                 { .f16, .bf16, .b32, .u32, .s32, .b64, .u64, .s64, .f32, .f64 };
//! ----------------------------------------------------------------
//! cp.reduce.async.bulk.dst.src.completion_mechanism{.level::cache_hint}.add.noftz.type [dstMem], [srcMem], size{, cache-policy};
//! .dst  =                 { .global };
//! .src  =                 { .shared::cta };
//! .completion_mechanism = { .bulk_group };
//! .type =                 { .f16, .bf16 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dst {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cluster").is_ok() {
                    return Ok(Dst::SharedCluster);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cluster"];
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
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".u32", ".s32", ".b64", ".u64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierCompleteTxBytes
            {
                let saved_pos = stream.position();
                if stream.expect_string(".mbarrier::complete_tx::bytes").is_ok() {
                    return Ok(CompletionMechanism::MbarrierCompleteTxBytes);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::complete_tx::bytes"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Src {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Src::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Redop::And);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Redop::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Redop::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Redop::Add);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Inc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".inc").is_ok() {
                    return Ok(Redop::Inc);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Dec
            {
                let saved_pos = stream.position();
                if stream.expect_string(".dec").is_ok() {
                    return Ok(Redop::Dec);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Redop::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Redop::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and", ".or", ".xor", ".add", ".inc", ".dec", ".min", ".max"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".reduce")?;
            let reduce = ();
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".bulk")?;
            let bulk = ();
            let dst = Dst::parse(stream)?;
            let src = Src::parse(stream)?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            let redop = Redop::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let dstmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let srcmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let mbar = AddressOperand::parse(stream)?;
            Ok(CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
                reduce,
                async_,
                bulk,
                dst,
                src,
                completion_mechanism,
                redop,
                type_,
                dstmem,
                srcmem,
                size,
                mbar,
            })
        }
    }


}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dst {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Dst::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
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
            // Try U64
            {
                let saved_pos = stream.position();
                if stream.expect_string(".u64").is_ok() {
                    return Ok(Type::U64);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".b32", ".u32", ".s32", ".b64", ".u64"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierCompleteTxBytes
            {
                let saved_pos = stream.position();
                if stream.expect_string(".mbarrier::complete_tx::bytes").is_ok() {
                    return Ok(CompletionMechanism::MbarrierCompleteTxBytes);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::complete_tx::bytes"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Src {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Src::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Redop {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try And
            {
                let saved_pos = stream.position();
                if stream.expect_string(".and").is_ok() {
                    return Ok(Redop::And);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Or
            {
                let saved_pos = stream.position();
                if stream.expect_string(".or").is_ok() {
                    return Ok(Redop::Or);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Xor
            {
                let saved_pos = stream.position();
                if stream.expect_string(".xor").is_ok() {
                    return Ok(Redop::Xor);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Add
            {
                let saved_pos = stream.position();
                if stream.expect_string(".add").is_ok() {
                    return Ok(Redop::Add);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Inc
            {
                let saved_pos = stream.position();
                if stream.expect_string(".inc").is_ok() {
                    return Ok(Redop::Inc);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Dec
            {
                let saved_pos = stream.position();
                if stream.expect_string(".dec").is_ok() {
                    return Ok(Redop::Dec);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Min
            {
                let saved_pos = stream.position();
                if stream.expect_string(".min").is_ok() {
                    return Ok(Redop::Min);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Max
            {
                let saved_pos = stream.position();
                if stream.expect_string(".max").is_ok() {
                    return Ok(Redop::Max);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".and", ".or", ".xor", ".add", ".inc", ".dec", ".min", ".max"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".reduce")?;
            let reduce = ();
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".bulk")?;
            let bulk = ();
            let dst = Dst::parse(stream)?;
            let src = Src::parse(stream)?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            let saved_pos = stream.position();
            let level_cache_hint = stream.expect_string(".level::cache_hint").is_ok();
            if !level_cache_hint {
                stream.set_position(saved_pos);
            }
            let redop = Redop::parse(stream)?;
            let type_ = Type::parse(stream)?;
            let dstmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let srcmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
                reduce,
                async_,
                bulk,
                dst,
                src,
                completion_mechanism,
                level_cache_hint,
                redop,
                type_,
                dstmem,
                srcmem,
                size,
                cache_policy,
            })
        }
    }


}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for Dst {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Dst::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Type {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try F16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".f16").is_ok() {
                    return Ok(Type::F16);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Bf16
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bf16").is_ok() {
                    return Ok(Type::Bf16);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".f16", ".bf16"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try BulkGroup
            {
                let saved_pos = stream.position();
                if stream.expect_string(".bulk_group").is_ok() {
                    return Ok(CompletionMechanism::BulkGroup);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bulk_group"];
            let found = stream.peek().map(|(t, _)| format!("{:?}", t)).unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Src {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Src::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
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

    impl PtxParser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".reduce")?;
            let reduce = ();
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_string(".bulk")?;
            let bulk = ();
            let dst = Dst::parse(stream)?;
            let src = Src::parse(stream)?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            let saved_pos = stream.position();
            let level_cache_hint = match LevelCacheHint::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_string(".add")?;
            let add = ();
            stream.expect_string(".noftz")?;
            let noftz = ();
            let type_ = Type::parse(stream)?;
            let dstmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let srcmem = AddressOperand::parse(stream)?;
            stream.expect(&PtxToken::Comma)?;
            let size = Operand::parse(stream)?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let cache_policy = match Operand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            Ok(CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
                reduce,
                async_,
                bulk,
                dst,
                src,
                completion_mechanism,
                level_cache_hint,
                add,
                noftz,
                type_,
                dstmem,
                srcmem,
                size,
                cache_policy,
            })
        }
    }


}

