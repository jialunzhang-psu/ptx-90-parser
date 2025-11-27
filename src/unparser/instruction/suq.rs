//! Original PTX specification:
//!
//! suq.query.b32   d, [a];
//! .query = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .array_size, .memory_layout };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::suq::section_0::*;

    impl PtxUnparser for SuqQueryB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "suq");
                    match &self.query {
                            Query::ChannelDataType => {
                                    push_directive(tokens, "channel_data_type");
                            }
                            Query::ChannelOrder => {
                                    push_directive(tokens, "channel_order");
                            }
                            Query::MemoryLayout => {
                                    push_directive(tokens, "memory_layout");
                            }
                            Query::ArraySize => {
                                    push_directive(tokens, "array_size");
                            }
                            Query::Height => {
                                    push_directive(tokens, "height");
                            }
                            Query::Width => {
                                    push_directive(tokens, "width");
                            }
                            Query::Depth => {
                                    push_directive(tokens, "depth");
                            }
                    }
                    push_directive(tokens, "b32");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

