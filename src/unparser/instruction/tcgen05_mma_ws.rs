//! Original PTX specification:
//!
//! // 1. Floating-point type without block scaling:
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem],  a-desc,  b-desc,  idesc,
//! enable-input-d {, zero-column-mask-desc };
//! tcgen05.mma.ws.cta_group::1.kind{.collector_usage}    [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc };
//! .kind = { .kind::f16, .kind::tf32, .kind::f8f6f4 };
//! ----------------------------------------------------------------------------------
//! // 2. Integer type:
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem],  a-desc,  b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! tcgen05.mma.ws.cta_group::1.kind::i8{.collector_usage} [d-tmem], [a-tmem], b-desc, idesc,
//! enable-input-d {, zero-column-mask-desc};
//! .collector_usage = { .collector::buffer::op };
//! ::buffer = { ::b0, ::b1, ::b2, ::b3 };
//! ::op   = { ::fill, ::use, ::lastuse, ::discard};

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma_ws::section_0::*;

    impl PtxUnparser for Tcgen05MmaWsCtaGroup1KindCollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "ws");
                    push_directive(tokens, "cta_group::1");
                    match &self.kind {
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                    }
                    if self.collector_usage {
                            push_directive(tokens, "collector_usage");
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.zero_column_mask_desc.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_0) = self.zero_column_mask_desc.as_ref() {
                        opt_0.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaWsCtaGroup1KindCollectorUsage1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "ws");
                    push_directive(tokens, "cta_group::1");
                    match &self.kind {
                            Kind::KindF8f6f4 => {
                                    push_directive(tokens, "kind::f8f6f4");
                            }
                            Kind::KindTf32 => {
                                    push_directive(tokens, "kind::tf32");
                            }
                            Kind::KindF16 => {
                                    push_directive(tokens, "kind::f16");
                            }
                    }
                    if self.collector_usage {
                            push_directive(tokens, "collector_usage");
                    }
                    self.d_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a_tmem.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.b_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.idesc.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.enable_input_d.unparse_tokens(tokens);
            if self.zero_column_mask_desc.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.zero_column_mask_desc.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::tcgen05_mma_ws::section_1::*;

    impl PtxUnparser for Tcgen05MmaWsCtaGroup1KindI8CollectorUsage {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "ws");
                    push_directive(tokens, "cta_group::1");
                    push_directive(tokens, "kind::i8");
                    if let Some(collector_usage_2) = self.collector_usage.as_ref() {
                            match collector_usage_2 {
                                    CollectorUsage::CollectorBufferOp(_, n1, n2) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "collector", combined).into()));
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
                    self.enable_input_d.unparse_tokens(tokens);
            if self.zero_column_mask_desc.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_3) = self.zero_column_mask_desc.as_ref() {
                        opt_3.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "mma");
                    push_directive(tokens, "ws");
                    push_directive(tokens, "cta_group::1");
                    push_directive(tokens, "kind::i8");
                    if let Some(collector_usage_4) = self.collector_usage.as_ref() {
                            match collector_usage_4 {
                                    CollectorUsage::CollectorBufferOp(_, n1, n2) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "collector", combined).into()));
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
                    self.enable_input_d.unparse_tokens(tokens);
            if self.zero_column_mask_desc.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.zero_column_mask_desc.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

