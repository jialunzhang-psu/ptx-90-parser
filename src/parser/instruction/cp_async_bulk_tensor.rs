//! Original PTX specification:
//!
//! // global -> shared::cta
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, cache-policy};
//! .dst =                  { .shared::cta };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! ----------------------------------------------------------------
//! // global -> shared::cluster
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.multicast}{.cta_group}{.level::cache_hint} [dstMem], [tensorMap, tensorCoords], [mbar]{, im2colInfo} {, ctaMask} {, cache-policy};
//! .dst =                  { .shared::cluster };
//! .src =                  { .global };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .mbarrier::complete_tx::bytes };
//! .cta_group =            { .cta_group::1, .cta_group::2 };
//! .load_mode =            { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =    { .L2::cache_hint };
//! .multicast =            { .multicast::cluster  };
//! ----------------------------------------------------------------
//! // shared::cta -> global;
//! cp.async.bulk.tensor.dim.dst.src{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {, cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .tile::scatter4, .im2col_no_offs };
//! .level::cache_hint =    { .L2::cache_hint };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_0::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierCompleteTxBytes
            {
                let saved_pos = stream.position();
                if stream
                    .expect_string(".mbarrier::complete_tx::bytes")
                    .is_ok()
                {
                    return Ok(CompletionMechanism::MbarrierCompleteTxBytes);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::complete_tx::bytes"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dim {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Dim::_1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Dim::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Dim::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".4d").is_ok() {
                    return Ok(Dim::_4d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".5d").is_ok() {
                    return Ok(Dim::_5d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".1d", ".2d", ".3d", ".4d", ".5d"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dst {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try SharedCta
            {
                let saved_pos = stream.position();
                if stream.expect_string(".shared::cta").is_ok() {
                    return Ok(Dst::SharedCta);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LoadMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Im2colW128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col::w::128").is_ok() {
                    return Ok(LoadMode::Im2colW128);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try TileGather4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile::gather4").is_ok() {
                    return Ok(LoadMode::TileGather4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Im2colW
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col::w").is_ok() {
                    return Ok(LoadMode::Im2colW);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Im2col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col").is_ok() {
                    return Ok(LoadMode::Im2col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Tile
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile").is_ok() {
                    return Ok(LoadMode::Tile);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".im2col::w::128",
                ".tile::gather4",
                ".im2col::w",
                ".im2col",
                ".tile",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Src {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Src::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_complete()?;
            stream.expect_string(".tensor")?;
            let tensor = ();
            stream.expect_complete()?;
            let dim = Dim::parse(stream)?;
            stream.expect_complete()?;
            let dst = Dst::parse(stream)?;
            stream.expect_complete()?;
            let src = Src::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let load_mode = match LoadMode::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let cta_group = match CtaGroup::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let dstmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tensormap = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let mbar = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let im2colinfo = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(
                CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
                    async_,
                    bulk,
                    tensor,
                    dim,
                    dst,
                    src,
                    load_mode,
                    completion_mechanism,
                    cta_group,
                    level_cache_hint,
                    dstmem,
                    tensormap,
                    mbar,
                    im2colinfo,
                    cache_policy,
                },
            )
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_1::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

    impl PtxParser for CompletionMechanism {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MbarrierCompleteTxBytes
            {
                let saved_pos = stream.position();
                if stream
                    .expect_string(".mbarrier::complete_tx::bytes")
                    .is_ok()
                {
                    return Ok(CompletionMechanism::MbarrierCompleteTxBytes);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".mbarrier::complete_tx::bytes"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CtaGroup {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try CtaGroup1
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::1").is_ok() {
                    return Ok(CtaGroup::CtaGroup1);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try CtaGroup2
            {
                let saved_pos = stream.position();
                if stream.expect_string(".cta_group::2").is_ok() {
                    return Ok(CtaGroup::CtaGroup2);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".cta_group::1", ".cta_group::2"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dim {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Dim::_1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Dim::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Dim::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".4d").is_ok() {
                    return Ok(Dim::_4d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".5d").is_ok() {
                    return Ok(Dim::_5d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".1d", ".2d", ".3d", ".4d", ".5d"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cluster"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LoadMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Im2colW128
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col::w::128").is_ok() {
                    return Ok(LoadMode::Im2colW128);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try TileGather4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile::gather4").is_ok() {
                    return Ok(LoadMode::TileGather4);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Im2colW
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col::w").is_ok() {
                    return Ok(LoadMode::Im2colW);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Im2col
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col").is_ok() {
                    return Ok(LoadMode::Im2col);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Tile
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile").is_ok() {
                    return Ok(LoadMode::Tile);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[
                ".im2col::w::128",
                ".tile::gather4",
                ".im2col::w",
                ".im2col",
                ".tile",
            ];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Multicast {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try MulticastCluster
            {
                let saved_pos = stream.position();
                if stream.expect_string(".multicast::cluster").is_ok() {
                    return Ok(Multicast::MulticastCluster);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".multicast::cluster"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Src {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try Global
            {
                let saved_pos = stream.position();
                if stream.expect_string(".global").is_ok() {
                    return Ok(Src::Global);
                }
                stream.set_position(saved_pos);
            }
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser
        for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint
    {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_complete()?;
            stream.expect_string(".tensor")?;
            let tensor = ();
            stream.expect_complete()?;
            let dim = Dim::parse(stream)?;
            stream.expect_complete()?;
            let dst = Dst::parse(stream)?;
            stream.expect_complete()?;
            let src = Src::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let load_mode = match LoadMode::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let multicast = match Multicast::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let cta_group = match CtaGroup::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            let dstmem = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let tensormap = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let mbar = AddressOperand::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let im2colinfo = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let has_comma = stream.expect(&PtxToken::Comma).is_ok();
            if !has_comma {
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            let ctamask = match GeneralOperand::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
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
            Ok(CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint {
                async_,
                bulk,
                tensor,
                dim,
                dst,
                src,
                load_mode,
                completion_mechanism,
                multicast,
                cta_group,
                level_cache_hint,
                dstmem,
                tensormap,
                mbar,
                im2colinfo,
                ctamask,
                cache_policy,
            })
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_2::*;

    // ============================================================================
    // Generated enum parsers
    // ============================================================================

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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".bulk_group"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for Dim {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try _1d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".1d").is_ok() {
                    return Ok(Dim::_1d);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try _2d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".2d").is_ok() {
                    return Ok(Dim::_2d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _3d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".3d").is_ok() {
                    return Ok(Dim::_3d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _4d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".4d").is_ok() {
                    return Ok(Dim::_4d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try _5d
            {
                let saved_pos = stream.position();
                if stream.expect_string(".5d").is_ok() {
                    return Ok(Dim::_5d);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".1d", ".2d", ".3d", ".4d", ".5d"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".global"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".L2::cache_hint"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for LoadMode {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            // Try TileScatter4
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile::scatter4").is_ok() {
                    return Ok(LoadMode::TileScatter4);
                }
                stream.set_position(saved_pos);
            }
            let saved_pos = stream.position();
            // Try Im2colNoOffs
            {
                let saved_pos = stream.position();
                if stream.expect_string(".im2col_no_offs").is_ok() {
                    return Ok(LoadMode::Im2colNoOffs);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let saved_pos = stream.position();
            // Try Tile
            {
                let saved_pos = stream.position();
                if stream.expect_string(".tile").is_ok() {
                    return Ok(LoadMode::Tile);
                }
                stream.set_position(saved_pos);
            }
            stream.set_position(saved_pos);
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".tile::scatter4", ".im2col_no_offs", ".tile"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
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
            let span = stream
                .peek()
                .map(|(_, s)| s.clone())
                .unwrap_or(Span { start: 0, end: 0 });
            let expected = &[".shared::cta"];
            let found = stream
                .peek()
                .map(|(t, _)| format!("{:?}", t))
                .unwrap_or_else(|_| "<end of input>".to_string());
            Err(crate::parser::unexpected_value(span, expected, found))
        }
    }

    impl PtxParser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
        fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
            stream.expect_string("cp")?;
            stream.expect_string(".async")?;
            let async_ = ();
            stream.expect_complete()?;
            stream.expect_string(".bulk")?;
            let bulk = ();
            stream.expect_complete()?;
            stream.expect_string(".tensor")?;
            let tensor = ();
            stream.expect_complete()?;
            let dim = Dim::parse(stream)?;
            stream.expect_complete()?;
            let dst = Dst::parse(stream)?;
            stream.expect_complete()?;
            let src = Src::parse(stream)?;
            stream.expect_complete()?;
            let saved_pos = stream.position();
            let load_mode = match LoadMode::parse(stream) {
                Ok(val) => Some(val),
                Err(_) => {
                    stream.set_position(saved_pos);
                    None
                }
            };
            stream.expect_complete()?;
            let completion_mechanism = CompletionMechanism::parse(stream)?;
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
            let tensormap = TexHandler2::parse(stream)?;
            stream.expect_complete()?;
            stream.expect(&PtxToken::Comma)?;
            let srcmem = AddressOperand::parse(stream)?;
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
            Ok(
                CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
                    async_,
                    bulk,
                    tensor,
                    dim,
                    dst,
                    src,
                    load_mode,
                    completion_mechanism,
                    level_cache_hint,
                    tensormap,
                    srcmem,
                    cache_policy,
                },
            )
        }
    }
}
