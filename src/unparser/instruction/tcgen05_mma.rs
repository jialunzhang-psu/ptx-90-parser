//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind   [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind   [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! ------------------------------------------------------------------
//! // 2. Floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .kind = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .cta_group      = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! ------------------------------------------------------------------
//! // 3. Convolution MMA for floating-point type without block scaling:
//! tcgen05.mma.cta_group.kind.collector_usage [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! tcgen05.mma.cta_group.kind.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d {, scale-input-d};
//! .kind      = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! .cta_group = { .cta_group::1,   .cta_group::2 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 4. Activation Stationary MMA for floating-point type with block scaling:
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem],  a-desc,  b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! tcgen05.mma.cta_group.kind.block_scale{.scale_vectorsize}.collector_usage
//! [d-tmem], [a-tmem], b-desc, idesc,
//! [scale-A-tmem], [scale-B-tmem], enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2 };
//! .scale_vectorsize  = { .scale_vec::1X, .scale_vec::2X, .scale_vec::4X, .block16, .block32 };
//! .kind            = { .kind::mxf8f6f4, .kind::mxf4, .kind::mxf4nvf4 };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };
//! ------------------------------------------------------------------
//! // 5. Integer type:
//! tcgen05.mma.cta_group.kind::i8  [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8  [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group = { .cta_group::1,   .cta_group::2  };
//! ------------------------------------------------------------------
//! // 6. Convolution MMA for integer type:
//! tcgen05.mma.cta_group.kind::i8.collector_usage          [d-tmem],  a-desc,  b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8.ashift{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! tcgen05.mma.cta_group.kind::i8{.ashift}.collector_usage [d-tmem], [a-tmem], b-desc, idesc
//! {, disable-output-lane }, enable-input-d;
//! .cta_group       = { .cta_group::1,   .cta_group::2  };
//! .collector_usage = { .collector::buffer::op };
//! ::buffer         = { ::a };
//! ::op             = { ::fill, ::use, ::lastuse, ::discard* };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_0::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKind {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_0) = self.disable_output_lane.as_ref() {
                        opt_0.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.scale_input_d.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.scale_input_d.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKind1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_2) = self.disable_output_lane.as_ref() {
                        opt_2.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.scale_input_d.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.scale_input_d.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_1::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindMxf8f6f4 => {
                                    push_directive(tokens, "kind::mxf8f6f4");
                            }
                            Kind::KindMxf4 => {
                                    push_directive(tokens, "kind::mxf4");
                            }
                            Kind::KindMxf4nvf4 => {
                                    push_directive(tokens, "kind::mxf4nvf4");
                            }
                    }
                    push_directive(tokens, "block_scale");
                    if let Some(scale_vectorsize_4) = self.scale_vectorsize.as_ref() {
                            match scale_vectorsize_4 {
                                    ScaleVectorsize::ScaleVec1x => {
                                            push_directive(tokens, "scale_vec::1X");
                                    }
                                    ScaleVectorsize::ScaleVec2x => {
                                            push_directive(tokens, "scale_vec::2X");
                                    }
                                    ScaleVectorsize::ScaleVec4x => {
                                            push_directive(tokens, "scale_vec::4X");
                                    }
                                    ScaleVectorsize::Block16 => {
                                            push_directive(tokens, "block16");
                                    }
                                    ScaleVectorsize::Block32 => {
                                            push_directive(tokens, "block32");
                                    }
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_b_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindMxf8f6f4 => {
                                    push_directive(tokens, "kind::mxf8f6f4");
                            }
                            Kind::KindMxf4 => {
                                    push_directive(tokens, "kind::mxf4");
                            }
                            Kind::KindMxf4nvf4 => {
                                    push_directive(tokens, "kind::mxf4nvf4");
                            }
                    }
                    push_directive(tokens, "block_scale");
                    if let Some(scale_vectorsize_5) = self.scale_vectorsize.as_ref() {
                            match scale_vectorsize_5 {
                                    ScaleVectorsize::ScaleVec1x => {
                                            push_directive(tokens, "scale_vec::1X");
                                    }
                                    ScaleVectorsize::ScaleVec2x => {
                                            push_directive(tokens, "scale_vec::2X");
                                    }
                                    ScaleVectorsize::ScaleVec4x => {
                                            push_directive(tokens, "scale_vec::4X");
                                    }
                                    ScaleVectorsize::Block16 => {
                                            push_directive(tokens, "block16");
                                    }
                                    ScaleVectorsize::Block32 => {
                                            push_directive(tokens, "block32");
                                    }
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_b_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_2 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_2::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKindCollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                    }
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_6) = self.disable_output_lane.as_ref() {
                        opt_6.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.scale_input_d.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.scale_input_d.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                    }
                    if self.ashift {
                            push_directive(tokens, "ashift");
                    }
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_8) = self.disable_output_lane.as_ref() {
                        opt_8.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.scale_input_d.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_9) = self.scale_input_d.as_ref() {
                        opt_9.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                    }
                    push_directive(tokens, "ashift");
                    if let Some(collector_usage_10) = self.collector_usage.as_ref() {
                            match collector_usage_10 {
                                    CollectorUsage::CollectorBufferOp(_, _, _) => {
                                            push_directive(tokens, "collector");
                                            push_token_from_str(tokens, "::a");
                                            push_token_from_str(tokens, "::fill");
                                            push_token_from_str(tokens, "::use");
                                            push_token_from_str(tokens, "::lastuse");
                                            push_token_from_str(tokens, "::discard*");
                                    }
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_11) = self.disable_output_lane.as_ref() {
                        opt_11.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.scale_input_d.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_12) = self.scale_input_d.as_ref() {
                        opt_12.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_3 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_3::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindMxf8f6f4 => {
                                    push_directive(tokens, "kind::mxf8f6f4");
                            }
                            Kind::KindMxf4 => {
                                    push_directive(tokens, "kind::mxf4");
                            }
                            Kind::KindMxf4nvf4 => {
                                    push_directive(tokens, "kind::mxf4nvf4");
                            }
                    }
                    push_directive(tokens, "block_scale");
                    if let Some(scale_vectorsize_13) = self.scale_vectorsize.as_ref() {
                            match scale_vectorsize_13 {
                                    ScaleVectorsize::ScaleVec1x => {
                                            push_directive(tokens, "scale_vec::1X");
                                    }
                                    ScaleVectorsize::ScaleVec2x => {
                                            push_directive(tokens, "scale_vec::2X");
                                    }
                                    ScaleVectorsize::ScaleVec4x => {
                                            push_directive(tokens, "scale_vec::4X");
                                    }
                                    ScaleVectorsize::Block16 => {
                                            push_directive(tokens, "block16");
                                    }
                                    ScaleVectorsize::Block32 => {
                                            push_directive(tokens, "block32");
                                    }
                            }
                    }
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_b_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.kind {
                            Kind::KindMxf8f6f4 => {
                                    push_directive(tokens, "kind::mxf8f6f4");
                            }
                            Kind::KindMxf4 => {
                                    push_directive(tokens, "kind::mxf4");
                            }
                            Kind::KindMxf4nvf4 => {
                                    push_directive(tokens, "kind::mxf4nvf4");
                            }
                    }
                    push_directive(tokens, "block_scale");
                    if let Some(scale_vectorsize_14) = self.scale_vectorsize.as_ref() {
                            match scale_vectorsize_14 {
                                    ScaleVectorsize::ScaleVec1x => {
                                            push_directive(tokens, "scale_vec::1X");
                                    }
                                    ScaleVectorsize::ScaleVec2x => {
                                            push_directive(tokens, "scale_vec::2X");
                                    }
                                    ScaleVectorsize::ScaleVec4x => {
                                            push_directive(tokens, "scale_vec::4X");
                                    }
                                    ScaleVectorsize::Block16 => {
                                            push_directive(tokens, "block16");
                                    }
                                    ScaleVectorsize::Block32 => {
                                            push_directive(tokens, "block32");
                                    }
                            }
                    }
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.scale_b_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_4 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_4::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKindI8 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "kind::i8");
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_15) = self.disable_output_lane.as_ref() {
                        opt_15.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindI81 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "kind::i8");
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_16) = self.disable_output_lane.as_ref() {
                        opt_16.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_5 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma::section_5::*;

    impl PtxUnparser for Tcgen05MmaCtaGroupKindI8CollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "kind::i8");
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.disable_output_lane.as_ref() {
                        opt_17.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "kind::i8");
                    push_directive(tokens, "ashift");
                    if let Some(collector_usage_18) = self.collector_usage.as_ref() {
                            match collector_usage_18 {
                                    CollectorUsage::CollectorBufferOp(_, _, _) => {
                                            push_directive(tokens, "collector");
                                            push_token_from_str(tokens, "::a");
                                            push_token_from_str(tokens, "::fill");
                                            push_token_from_str(tokens, "::use");
                                            push_token_from_str(tokens, "::lastuse");
                                            push_token_from_str(tokens, "::discard*");
                                    }
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_19) = self.disable_output_lane.as_ref() {
                        opt_19.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "kind::i8");
                    if self.ashift {
                            push_directive(tokens, "ashift");
                    }
                    match &self.collector_usage {
                            CollectorUsage::CollectorBufferOp(_, _, _) => {
                                    push_directive(tokens, "collector");
                                    push_token_from_str(tokens, "::a");
                                    push_token_from_str(tokens, "::fill");
                                    push_token_from_str(tokens, "::use");
                                    push_token_from_str(tokens, "::lastuse");
                                    push_token_from_str(tokens, "::discard*");
                            }
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            if self.disable_output_lane.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_20) = self.disable_output_lane.as_ref() {
                        opt_20.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

