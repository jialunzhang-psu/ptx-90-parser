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
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_0::*;

    impl PtxUnparser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
            push_directive(tokens, "tensor");
            match &self.dim {
                Dim::_1d => {
                    push_directive(tokens, "1d");
                }
                Dim::_2d => {
                    push_directive(tokens, "2d");
                }
                Dim::_3d => {
                    push_directive(tokens, "3d");
                }
                Dim::_4d => {
                    push_directive(tokens, "4d");
                }
                Dim::_5d => {
                    push_directive(tokens, "5d");
                }
            }
            match &self.dst {
                Dst::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
            }
            match &self.src {
                Src::Global => {
                    push_directive(tokens, "global");
                }
            }
            if let Some(load_mode_0) = self.load_mode.as_ref() {
                match load_mode_0 {
                    LoadMode::Im2colW128 => {
                        push_directive(tokens, "im2col::w::128");
                    }
                    LoadMode::TileGather4 => {
                        push_directive(tokens, "tile::gather4");
                    }
                    LoadMode::Im2colW => {
                        push_directive(tokens, "im2col::w");
                    }
                    LoadMode::Im2col => {
                        push_directive(tokens, "im2col");
                    }
                    LoadMode::Tile => {
                        push_directive(tokens, "tile");
                    }
                }
            }
            match &self.completion_mechanism {
                CompletionMechanism::MbarrierCompleteTxBytes => {
                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                }
            }
            if let Some(cta_group_1) = self.cta_group.as_ref() {
                match cta_group_1 {
                    CtaGroup::CtaGroup1 => {
                        push_directive(tokens, "cta_group::1");
                    }
                    CtaGroup::CtaGroup2 => {
                        push_directive(tokens, "cta_group::2");
                    }
                }
            }
            if let Some(level_cache_hint_2) = self.level_cache_hint.as_ref() {
                match level_cache_hint_2 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dstmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.tensormap.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.mbar.unparse_tokens_mode(tokens, spaced);
            if self.im2colinfo.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_3) = self.im2colinfo.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_3.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_4) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_4.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_1::*;

    impl PtxUnparser
        for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint
    {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
            push_directive(tokens, "tensor");
            match &self.dim {
                Dim::_1d => {
                    push_directive(tokens, "1d");
                }
                Dim::_2d => {
                    push_directive(tokens, "2d");
                }
                Dim::_3d => {
                    push_directive(tokens, "3d");
                }
                Dim::_4d => {
                    push_directive(tokens, "4d");
                }
                Dim::_5d => {
                    push_directive(tokens, "5d");
                }
            }
            match &self.dst {
                Dst::SharedCluster => {
                    push_directive(tokens, "shared::cluster");
                }
            }
            match &self.src {
                Src::Global => {
                    push_directive(tokens, "global");
                }
            }
            if let Some(load_mode_5) = self.load_mode.as_ref() {
                match load_mode_5 {
                    LoadMode::Im2colW128 => {
                        push_directive(tokens, "im2col::w::128");
                    }
                    LoadMode::TileGather4 => {
                        push_directive(tokens, "tile::gather4");
                    }
                    LoadMode::Im2colW => {
                        push_directive(tokens, "im2col::w");
                    }
                    LoadMode::Im2col => {
                        push_directive(tokens, "im2col");
                    }
                    LoadMode::Tile => {
                        push_directive(tokens, "tile");
                    }
                }
            }
            match &self.completion_mechanism {
                CompletionMechanism::MbarrierCompleteTxBytes => {
                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                }
            }
            if let Some(multicast_6) = self.multicast.as_ref() {
                match multicast_6 {
                    Multicast::MulticastCluster => {
                        push_directive(tokens, "multicast::cluster");
                    }
                }
            }
            if let Some(cta_group_7) = self.cta_group.as_ref() {
                match cta_group_7 {
                    CtaGroup::CtaGroup1 => {
                        push_directive(tokens, "cta_group::1");
                    }
                    CtaGroup::CtaGroup2 => {
                        push_directive(tokens, "cta_group::2");
                    }
                }
            }
            if let Some(level_cache_hint_8) = self.level_cache_hint.as_ref() {
                match level_cache_hint_8 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.dstmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.tensormap.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.mbar.unparse_tokens_mode(tokens, spaced);
            if self.im2colinfo.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_9) = self.im2colinfo.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_9.unparse_tokens_mode(tokens, spaced);
            }
            if self.ctamask.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_10) = self.ctamask.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_10.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_11) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_11.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_2::*;

    impl PtxUnparser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
            push_directive(tokens, "tensor");
            match &self.dim {
                Dim::_1d => {
                    push_directive(tokens, "1d");
                }
                Dim::_2d => {
                    push_directive(tokens, "2d");
                }
                Dim::_3d => {
                    push_directive(tokens, "3d");
                }
                Dim::_4d => {
                    push_directive(tokens, "4d");
                }
                Dim::_5d => {
                    push_directive(tokens, "5d");
                }
            }
            match &self.dst {
                Dst::Global => {
                    push_directive(tokens, "global");
                }
            }
            match &self.src {
                Src::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
            }
            if let Some(load_mode_12) = self.load_mode.as_ref() {
                match load_mode_12 {
                    LoadMode::TileScatter4 => {
                        push_directive(tokens, "tile::scatter4");
                    }
                    LoadMode::Im2colNoOffs => {
                        push_directive(tokens, "im2col_no_offs");
                    }
                    LoadMode::Tile => {
                        push_directive(tokens, "tile");
                    }
                }
            }
            match &self.completion_mechanism {
                CompletionMechanism::BulkGroup => {
                    push_directive(tokens, "bulk_group");
                }
            }
            if let Some(level_cache_hint_13) = self.level_cache_hint.as_ref() {
                match level_cache_hint_13 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.tensormap.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.srcmem.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_14) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_14.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
