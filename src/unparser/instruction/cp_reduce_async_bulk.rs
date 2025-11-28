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
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_0::*;

    impl PtxUnparser for CpReduceAsyncBulkDstSrcCompletionMechanismRedopType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "reduce");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
            match &self.dst {
                Dst::SharedCluster => {
                    push_directive(tokens, "shared::cluster");
                }
            }
            match &self.src {
                Src::SharedCta => {
                    push_directive(tokens, "shared::cta");
                }
            }
            match &self.completion_mechanism {
                CompletionMechanism::MbarrierCompleteTxBytes => {
                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                }
            }
            match &self.redop {
                Redop::And => {
                    push_directive(tokens, "and");
                }
                Redop::Xor => {
                    push_directive(tokens, "xor");
                }
                Redop::Add => {
                    push_directive(tokens, "add");
                }
                Redop::Inc => {
                    push_directive(tokens, "inc");
                }
                Redop::Dec => {
                    push_directive(tokens, "dec");
                }
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
                Redop::Or => {
                    push_directive(tokens, "or");
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
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
            self.srcmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.size.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.mbar.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::cp_reduce_async_bulk::section_1::*;

    impl PtxUnparser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "reduce");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
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
            match &self.completion_mechanism {
                CompletionMechanism::MbarrierCompleteTxBytes => {
                    push_directive(tokens, "mbarrier::complete_tx::bytes");
                }
            }
            if self.level_cache_hint {
                push_directive(tokens, "level::cache_hint");
            }
            match &self.redop {
                Redop::And => {
                    push_directive(tokens, "and");
                }
                Redop::Xor => {
                    push_directive(tokens, "xor");
                }
                Redop::Add => {
                    push_directive(tokens, "add");
                }
                Redop::Inc => {
                    push_directive(tokens, "inc");
                }
                Redop::Dec => {
                    push_directive(tokens, "dec");
                }
                Redop::Min => {
                    push_directive(tokens, "min");
                }
                Redop::Max => {
                    push_directive(tokens, "max");
                }
                Redop::Or => {
                    push_directive(tokens, "or");
                }
            }
            match &self.type_ {
                Type::B32 => {
                    push_directive(tokens, "b32");
                }
                Type::U32 => {
                    push_directive(tokens, "u32");
                }
                Type::S32 => {
                    push_directive(tokens, "s32");
                }
                Type::B64 => {
                    push_directive(tokens, "b64");
                }
                Type::U64 => {
                    push_directive(tokens, "u64");
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
            self.srcmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.size.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_0) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_0.unparse_tokens_mode(tokens, spaced);
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
    use crate::r#type::instruction::cp_reduce_async_bulk::section_2::*;

    impl PtxUnparser for CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "cp");
            push_directive(tokens, "reduce");
            push_directive(tokens, "async");
            push_directive(tokens, "bulk");
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
            match &self.completion_mechanism {
                CompletionMechanism::BulkGroup => {
                    push_directive(tokens, "bulk_group");
                }
            }
            if let Some(level_cache_hint_1) = self.level_cache_hint.as_ref() {
                match level_cache_hint_1 {
                    LevelCacheHint::L2CacheHint => {
                        push_directive(tokens, "L2::cache_hint");
                    }
                }
            }
            push_directive(tokens, "add");
            push_directive(tokens, "noftz");
            match &self.type_ {
                Type::Bf16 => {
                    push_directive(tokens, "bf16");
                }
                Type::F16 => {
                    push_directive(tokens, "f16");
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
            self.srcmem.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
            if spaced {
                tokens.push(PtxToken::Space);
            }
            self.size.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_2) = self.cache_policy.as_ref() {
                if spaced {
                    tokens.push(PtxToken::Space);
                }
                opt_2.unparse_tokens_mode(tokens, spaced);
            }
            tokens.push(PtxToken::Semicolon);
            if spaced {
                tokens.push(PtxToken::Newline);
            }
        }
    }
}
