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
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::atom::section_0::*;

    impl PtxUnparser for AtomSemScopeSpaceOpLevelCacheHintType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_0) = self.sem.as_ref() {
                            match sem_0 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_1) = self.scope.as_ref() {
                            match scope_1 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_2) = self.space.as_ref() {
                            match space_2 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.op {
                            Op::Exch => {
                                    push_directive(tokens, "exch");
                            }
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Xor => {
                                    push_directive(tokens, "xor");
                            }
                            Op::Cas => {
                                    push_directive(tokens, "cas");
                            }
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Inc => {
                                    push_directive(tokens, "inc");
                            }
                            Op::Dec => {
                                    push_directive(tokens, "dec");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
                    if let Some(level_cache_hint_3) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_3 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_4.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_5) = self.sem.as_ref() {
                            match sem_5 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_6) = self.scope.as_ref() {
                            match scope_6 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_7) = self.space.as_ref() {
                            match space_7 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.op {
                            Op::Exch => {
                                    push_directive(tokens, "exch");
                            }
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Xor => {
                                    push_directive(tokens, "xor");
                            }
                            Op::Cas => {
                                    push_directive(tokens, "cas");
                            }
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Inc => {
                                    push_directive(tokens, "inc");
                            }
                            Op::Dec => {
                                    push_directive(tokens, "dec");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Type::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Type::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Type::S64 => {
                                    push_directive(tokens, "s64");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceCasB16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_8) = self.sem.as_ref() {
                            match sem_8 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_9) = self.scope.as_ref() {
                            match scope_9 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_10) = self.space.as_ref() {
                            match space_10 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "cas");
                    push_directive(tokens, "b16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceCasB128 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_11) = self.sem.as_ref() {
                            match sem_11 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_12) = self.scope.as_ref() {
                            match scope_12 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_13) = self.space.as_ref() {
                            match space_13 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "cas");
                    push_directive(tokens, "b128");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceExchLevelCacheHintB128 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_14) = self.sem.as_ref() {
                            match sem_14 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_15) = self.scope.as_ref() {
                            match scope_15 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_16) = self.space.as_ref() {
                            match space_16 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "exch");
                    if let Some(level_cache_hint_17) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_17 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "b128");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_18) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_18.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_19) = self.sem.as_ref() {
                            match sem_19 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_20) = self.scope.as_ref() {
                            match scope_20 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_21) = self.space.as_ref() {
                            match space_21 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "add");
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_22) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_22 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "f16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_23) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_23.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_24) = self.sem.as_ref() {
                            match sem_24 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_25) = self.scope.as_ref() {
                            match scope_25 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_26) = self.space.as_ref() {
                            match space_26 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "add");
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_27) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_27 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "f16x2");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_28) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_28.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_29) = self.sem.as_ref() {
                            match sem_29 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_30) = self.scope.as_ref() {
                            match scope_30 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_31) = self.space.as_ref() {
                            match space_31 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "add");
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_32) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_32 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_33) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_33.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_34) = self.sem.as_ref() {
                            match sem_34 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_35) = self.scope.as_ref() {
                            match scope_35 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if let Some(space_36) = self.space.as_ref() {
                            match space_36 {
                                    Space::SharedCluster => {
                                            push_directive(tokens, "shared::cluster");
                                    }
                                    Space::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Space::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Space::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    push_directive(tokens, "add");
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_37) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_37 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    push_directive(tokens, "bf16x2");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_38) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_38.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::atom::section_1::*;

    impl PtxUnparser for AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_39) = self.sem.as_ref() {
                            match sem_39 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_40) = self.scope.as_ref() {
                            match scope_40 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    push_directive(tokens, "add");
                    if let Some(level_cache_hint_41) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_41 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_32_bit {
                            Vec32Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec32Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    push_directive(tokens, "f32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_42) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_42.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_43) = self.sem.as_ref() {
                            match sem_43 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_44) = self.scope.as_ref() {
                            match scope_44 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_45) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_45 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_16_bit {
                            Vec16Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec16Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                            Vec16Bit::V8 => {
                                    push_directive(tokens, "v8");
                            }
                    }
                    match &self.half_word_type {
                            HalfWordType::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                            HalfWordType::F16 => {
                                    push_directive(tokens, "f16");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_46) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_46.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "atom");
                    if let Some(sem_47) = self.sem.as_ref() {
                            match sem_47 {
                                    Sem::Relaxed => {
                                            push_directive(tokens, "relaxed");
                                    }
                                    Sem::Acquire => {
                                            push_directive(tokens, "acquire");
                                    }
                                    Sem::Release => {
                                            push_directive(tokens, "release");
                                    }
                                    Sem::AcqRel => {
                                            push_directive(tokens, "acq_rel");
                                    }
                            }
                    }
                    if let Some(scope_48) = self.scope.as_ref() {
                            match scope_48 {
                                    Scope::Cluster => {
                                            push_directive(tokens, "cluster");
                                    }
                                    Scope::Cta => {
                                            push_directive(tokens, "cta");
                                    }
                                    Scope::Gpu => {
                                            push_directive(tokens, "gpu");
                                    }
                                    Scope::Sys => {
                                            push_directive(tokens, "sys");
                                    }
                            }
                    }
                    if self.global {
                            push_directive(tokens, "global");
                    }
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                    }
                    push_directive(tokens, "noftz");
                    if let Some(level_cache_hint_49) = self.level_cache_hint.as_ref() {
                            match level_cache_hint_49 {
                                    LevelCacheHint::L2CacheHint => {
                                            push_directive(tokens, "L2::cache_hint");
                                    }
                            }
                    }
                    match &self.vec_32_bit {
                            Vec32Bit::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec32Bit::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    match &self.packed_type {
                            PackedType::Bf16x2 => {
                                    push_directive(tokens, "bf16x2");
                            }
                            PackedType::F16x2 => {
                                    push_directive(tokens, "f16x2");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
            if self.cache_policy.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_50) = self.cache_policy.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_50.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

