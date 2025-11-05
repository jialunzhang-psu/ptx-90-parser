//! Original PTX specification:
//!
//! // Floating point format .f16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f16, .s8, .u8};
//! .btype  = {.f16, .s8, .u8};
//! .ctype  = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! // Alternate floating point format .bf16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.bf16 };
//! .btype  = {.bf16 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Alternate floating point format .tf32 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.tf32 };
//! .btype  = {.tf32 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Double precision Floating point .f64 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f64 };
//! .btype  = {.f64 };
//! .ctype  = {.f64 };
//! ----------------------------------------------------------------
//! // Sub-byte loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! .ctype  = {.s32};
//! ----------------------------------------------------------------
//! // Single-bit loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .ctype  = {.s32};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_0::*;

    impl PtxUnparser for WmmaLoadASyncAlignedLayoutShapeSsAtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
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
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Atype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Atype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.stride.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedLayoutShapeSsBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
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
                    if let Some(ss_2) = self.ss.as_ref() {
                            match ss_2 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Btype::S8 => {
                                    push_directive(tokens, "s8");
                            }
                            Btype::U8 => {
                                    push_directive(tokens, "u8");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.stride.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                    if let Some(ss_4) = self.ss.as_ref() {
                            match ss_4 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.stride.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_1::*;

    impl PtxUnparser for WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
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
                    if let Some(ss_6) = self.ss.as_ref() {
                            match ss_6 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.stride.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
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
                    if let Some(ss_8) = self.ss.as_ref() {
                            match ss_8 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::Bf16 => {
                                    push_directive(tokens, "bf16");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_9) = self.stride.as_ref() {
                        opt_9.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                    if let Some(ss_10) = self.ss.as_ref() {
                            match ss_10 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_11) = self.stride.as_ref() {
                        opt_11.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_2::*;

    impl PtxUnparser for WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
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
                    if let Some(ss_12) = self.ss.as_ref() {
                            match ss_12 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::Tf32 => {
                                    push_directive(tokens, "tf32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_13) = self.stride.as_ref() {
                        opt_13.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
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
                    if let Some(ss_14) = self.ss.as_ref() {
                            match ss_14 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::Tf32 => {
                                    push_directive(tokens, "tf32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_15) = self.stride.as_ref() {
                        opt_15.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                    if let Some(ss_16) = self.ss.as_ref() {
                            match ss_16 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.stride.as_ref() {
                        opt_17.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_3::*;

    impl PtxUnparser for WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
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
                    if let Some(ss_18) = self.ss.as_ref() {
                            match ss_18 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_19) = self.stride.as_ref() {
                        opt_19.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
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
                    if let Some(ss_20) = self.ss.as_ref() {
                            match ss_20 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_21) = self.stride.as_ref() {
                        opt_21.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                    if let Some(ss_22) = self.ss.as_ref() {
                            match ss_22 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_23) = self.stride.as_ref() {
                        opt_23.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_4::*;

    impl PtxUnparser for WmmaLoadASyncAlignedRowShapeSsAtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "row");
                    match &self.shape {
                            Shape::M8n8k32 => {
                                    push_directive(tokens, "m8n8k32");
                            }
                    }
                    if let Some(ss_24) = self.ss.as_ref() {
                            match ss_24 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::S4 => {
                                    push_directive(tokens, "s4");
                            }
                            Atype::U4 => {
                                    push_directive(tokens, "u4");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_25) = self.stride.as_ref() {
                        opt_25.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedColShapeSsBtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "col");
                    match &self.shape {
                            Shape::M8n8k32 => {
                                    push_directive(tokens, "m8n8k32");
                            }
                    }
                    if let Some(ss_26) = self.ss.as_ref() {
                            match ss_26 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::S4 => {
                                    push_directive(tokens, "s4");
                            }
                            Btype::U4 => {
                                    push_directive(tokens, "u4");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_27) = self.stride.as_ref() {
                        opt_27.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                            Shape::M8n8k32 => {
                                    push_directive(tokens, "m8n8k32");
                            }
                    }
                    if let Some(ss_28) = self.ss.as_ref() {
                            match ss_28 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_29) = self.stride.as_ref() {
                        opt_29.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::wmma_load::section_5::*;

    impl PtxUnparser for WmmaLoadASyncAlignedRowShapeSsAtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "a");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "row");
                    match &self.shape {
                            Shape::M8n8k128 => {
                                    push_directive(tokens, "m8n8k128");
                            }
                    }
                    if let Some(ss_30) = self.ss.as_ref() {
                            match ss_30 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.atype {
                            Atype::B1 => {
                                    push_directive(tokens, "b1");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_31) = self.stride.as_ref() {
                        opt_31.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadBSyncAlignedColShapeSsBtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "b");
                    push_directive(tokens, "sync");
                    push_directive(tokens, "aligned");
                    push_directive(tokens, "col");
                    match &self.shape {
                            Shape::M8n8k128 => {
                                    push_directive(tokens, "m8n8k128");
                            }
                    }
                    if let Some(ss_32) = self.ss.as_ref() {
                            match ss_32 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.btype {
                            Btype::B1 => {
                                    push_directive(tokens, "b1");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_33) = self.stride.as_ref() {
                        opt_33.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "wmma");
                    push_directive(tokens, "load");
                    push_directive(tokens, "c");
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
                    }
                    if let Some(ss_34) = self.ss.as_ref() {
                            match ss_34 {
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                                    Ss::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    self.r.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.p.unparse_tokens(tokens);
            if self.stride.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_35) = self.stride.as_ref() {
                        opt_35.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

