//! Original PTX specification:
//!
//! // global -> shared::cluster:
//! cp.async.bulk.prefetch.tensor.dim.L2.src{.load_mode}{.level::cache_hint} [tensorMap, tensorCoords] {, im2colInfo } {, cache-policy};
//! .src =                { .global };
//! .dim =                { .1d, .2d, .3d, .4d, .5d };
//! .load_mode =          { .tile, .tile::gather4, .im2col, .im2col::w, .im2col::w::128 };
//! .level::cache_hint =  { .L2::cache_hint };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_bulk_prefetch_tensor::section_0::*;

    impl PtxUnparser for CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
            push_directive(tokens, "prefetch");
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
            push_directive(tokens, "L2");
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
            if let Some(level_cache_hint_1) = self.level_cache_hint.as_ref() {
                match level_cache_hint_1 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.tensormap.unparse_tokens_mode(tokens, spaced);
            if self.im2colinfo.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_2) = self.im2colinfo.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_2.unparse_tokens_mode(tokens, spaced);
            }
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_3) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_3.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
