//! Original PTX specification:
//!
//! // shared::cta -> global
//! cp.reduce.async.bulk.tensor.dim.dst.src.redOp{.load_mode}.completion_mechanism{.level::cache_hint} [tensorMap, tensorCoords], [srcMem] {,cache-policy};
//! .dst =                  { .global };
//! .src =                  { .shared::cta };
//! .dim =                  { .1d, .2d, .3d, .4d, .5d };
//! .completion_mechanism = { .bulk_group };
//! .load_mode =            { .tile, .im2col_no_offs };
//! .redOp =                { .add, .min, .max, .inc, .dec, .and, .or, .xor};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk_tensor::section_0::*;

    impl PtxUnparser
        for CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint
    {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "reduce");
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
            match &self.redop {
                Redop::Add => {
                    push_directive(tokens, "add");
                }
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
                Redop::Inc => {
                    push_directive(tokens, "inc");
                }
                Redop::Dec => {
                    push_directive(tokens, "dec");
                }
                Redop::And => {
                    push_directive(tokens, "and");
                }
                Redop::Xor => {
                    push_directive(tokens, "xor");
                }
                Redop::Or => {
                    push_directive(tokens, "or");
                }
            }
            if let Some(load_mode_0) = self.load_mode.as_ref() {
                match load_mode_0 {
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
            if self.level_cache_hint {
                push_directive(tokens, "level::cache_hint");
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
            if let Some(opt_1) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_1.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
