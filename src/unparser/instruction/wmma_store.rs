//! Original PTX specification:
//!
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32, .m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f64};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_0::*;

    impl PtxUnparser for WmmaStoreDSyncAlignedLayoutShapeSsType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "store");
                    push_directive(tokens, "d");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.layout {
                            Layout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Layout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M16n16k16 => {
                                    push_directive(tokens, "m16n16k16");
                            }
                            Shape::M8n32k16 => {
                                    push_directive(tokens, "m8n32k16");
                            }
                            Shape::M32n8k16 => {
                                    push_directive(tokens, "m32n8k16");
                            }
                    }
                    if let Some(ss_0) = self.ss.as_ref() {
                            match ss_0 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.stride.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_1::*;

    impl PtxUnparser for WmmaStoreDSyncAlignedLayoutShapeSsType1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "store");
                    push_directive(tokens, "d");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.layout {
                            Layout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Layout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M8n8k128 => {
                                    push_directive(tokens, "m8n8k128");
                            }
                            Shape::M8n8k32 => {
                                    push_directive(tokens, "m8n8k32");
                            }
                    }
                    if let Some(ss_2) = self.ss.as_ref() {
                            match ss_2 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.stride.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_2::*;

    impl PtxUnparser for WmmaStoreDSyncAlignedLayoutShapeSsType2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "store");
                    push_directive(tokens, "d");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.layout {
                            Layout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Layout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M16n16k8 => {
                                    push_directive(tokens, "m16n16k8");
                            }
                    }
                    if let Some(ss_4) = self.ss.as_ref() {
                            match ss_4 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.stride.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_store::section_3::*;

    impl PtxUnparser for WmmaStoreDSyncAlignedLayoutShapeSsType3 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "store");
                    push_directive(tokens, "d");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    match &self.layout {
                            Layout::Row => {
                                    push_directive(tokens, "row");
                            }
                            Layout::Col => {
                                    push_directive(tokens, "col");
                            }
                    }
                    match &self.shape {
                            Shape::M8n8k4 => {
                                    push_directive(tokens, "m8n8k4");
                            }
                    }
                    if let Some(ss_6) = self.ss.as_ref() {
                            match ss_6 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                            }
                    }
                    match &self.type_ {
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.r.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.stride.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

