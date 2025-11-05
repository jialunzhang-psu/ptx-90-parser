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
                                    LoadMode::Tile => {
                                            push_directive(tokens, "tile");
                                    }
                                    LoadMode::TileGather4 => {
                                            push_directive(tokens, "tile::gather4");
                                    }
                                    LoadMode::Im2col => {
                                            push_directive(tokens, "im2col");
                                    }
                                    LoadMode::Im2colW => {
                                            push_directive(tokens, "im2col::w");
                                    }
                                    LoadMode::Im2colW128 => {
                                            push_directive(tokens, "im2col::w::128");
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
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_3_0, ref group_3_1) = &self.tensormap;
                    group_3_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            if self.im2colinfo.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.im2colinfo.as_ref() {
                        opt_4.unparse_tokens(tokens);
                    }
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.cache_policy.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_1::*;

    impl PtxUnparser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
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
                    if let Some(load_mode_6) = self.load_mode.as_ref() {
                            match load_mode_6 {
                                    LoadMode::Tile => {
                                            push_directive(tokens, "tile");
                                    }
                                    LoadMode::TileGather4 => {
                                            push_directive(tokens, "tile::gather4");
                                    }
                                    LoadMode::Im2col => {
                                            push_directive(tokens, "im2col");
                                    }
                                    LoadMode::Im2colW => {
                                            push_directive(tokens, "im2col::w");
                                    }
                                    LoadMode::Im2colW128 => {
                                            push_directive(tokens, "im2col::w::128");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::MbarrierCompleteTxBytes => {
                                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                            }
                    }
                    if let Some(multicast_7) = self.multicast.as_ref() {
                            match multicast_7 {
                                    Multicast::MulticastCluster => {
                                            push_directive(tokens, "multicast::cluster");
                                    }
                            }
                    }
                    if let Some(cta_group_8) = self.cta_group.as_ref() {
                            match cta_group_8 {
                                    CtaGroup::CtaGroup1 => {
                                            push_directive(tokens, "cta_group::1");
                                    }
                                    CtaGroup::CtaGroup2 => {
                                            push_directive(tokens, "cta_group::2");
                                    }
                            }
                    }
                    if let Some(level_cache_hint_9) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_9 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    self.dstmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_10_0, ref group_10_1) = &self.tensormap;
                    group_10_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_10_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.mbar.unparse_tokens(tokens);
            if self.im2colinfo.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_11) = self.im2colinfo.as_ref() {
                        opt_11.unparse_tokens(tokens);
                    }
            if self.ctamask.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_12) = self.ctamask.as_ref() {
                        opt_12.unparse_tokens(tokens);
                    }
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_13) = self.cache_policy.as_ref() {
                        opt_13.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_tensor::section_2::*;

    impl PtxUnparser for CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
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
                    if let Some(load_mode_14) = self.load_mode.as_ref() {
                            match load_mode_14 {
                                    LoadMode::Tile => {
                                            push_directive(tokens, "tile");
                                    }
                                    LoadMode::TileScatter4 => {
                                            push_directive(tokens, "tile::scatter4");
                                    }
                                    LoadMode::Im2colNoOffs => {
                                            push_directive(tokens, "im2col_no_offs");
                                    }
                            }
                    }
                    match &self.completion_mechanism {
                            CompletionMechanism::BulkGroup => {
                                    push_directive(tokens, "bulk_group");
                            }
                    }
                    if let Some(level_cache_hint_15) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_15 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_16_0, ref group_16_1) = &self.tensormap;
                    group_16_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_16_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.srcmem.unparse_tokens(tokens);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.cache_policy.as_ref() {
                        opt_17.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

