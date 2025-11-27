//! Original PTX specification:
//!
//! tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
//! tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
//! tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;
//! .mode    = { .tile };
//! .field1  = { .global_address, .rank };
//! .field2  = { .box_dim, .global_dim, .global_stride, .element_stride  };
//! .field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode };
//! .ss      = { .global, .shared::cta };
//! .type    = { .b32, .b64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tensormap_replace::section_0::*;

    impl PtxUnparser for TensormapReplaceModeField1SsB1024Type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tensormap");
                    push_directive(tokens, "replace");
                    match &self.mode {
                            Mode::Tile => {
                                    push_directive(tokens, "tile");
                            }
                    }
                    match &self.field1 {
                            Field1::GlobalAddress => {
                                    push_directive(tokens, "global_address");
                            }
                            Field1::Rank => {
                                    push_directive(tokens, "rank");
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
                            }
                    }
                    push_directive(tokens, "b1024");
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.new_val.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TensormapReplaceModeField2SsB1024Type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tensormap");
                    push_directive(tokens, "replace");
                    match &self.mode {
                            Mode::Tile => {
                                    push_directive(tokens, "tile");
                            }
                    }
                    match &self.field2 {
                            Field2::ElementStride => {
                                    push_directive(tokens, "element_stride");
                            }
                            Field2::GlobalStride => {
                                    push_directive(tokens, "global_stride");
                            }
                            Field2::GlobalDim => {
                                    push_directive(tokens, "global_dim");
                            }
                            Field2::BoxDim => {
                                    push_directive(tokens, "box_dim");
                            }
                    }
                    if let Some(ss_1) = self.ss.as_ref() {
                            match ss_1 {
                                    Ss::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                                    Ss::Global => {
                                            push_directive(tokens, "global");
                                    }
                            }
                    }
                    push_directive(tokens, "b1024");
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.ord.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.new_val.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TensormapReplaceModeField3SsB1024Type {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tensormap");
                    push_directive(tokens, "replace");
                    match &self.mode {
                            Mode::Tile => {
                                    push_directive(tokens, "tile");
                            }
                    }
                    match &self.field3 {
                            Field3::InterleaveLayout => {
                                    push_directive(tokens, "interleave_layout");
                            }
                            Field3::SwizzleAtomicity => {
                                    push_directive(tokens, "swizzle_atomicity");
                            }
                            Field3::SwizzleMode => {
                                    push_directive(tokens, "swizzle_mode");
                            }
                            Field3::FillMode => {
                                    push_directive(tokens, "fill_mode");
                            }
                            Field3::Elemtype => {
                                    push_directive(tokens, "elemtype");
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
                            }
                    }
                    push_directive(tokens, "b1024");
                    match &self.type_ {
                            Type::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Type::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.addr.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.new_val.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

