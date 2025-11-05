//! Original PTX specification:
//!
//! txq.tquery.b32         d, [a];       // texture attributes
//! txq.level.tlquery.b32  d, [a], lod;  // texture attributes
//! txq.squery.b32         d, [a];       // sampler attributes
//! .tquery  = { .width, .height, .depth,
//! .channel_data_type, .channel_order,
//! .normalized_coords, .array_size,
//! .num_mipmap_levels, .num_samples};
//! .tlquery = { .width, .height, .depth };
//! .squery  = { .force_unnormalized_coords, .filter_mode,
//! .addr_mode_0, addr_mode_1, addr_mode_2 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::txq::section_0::*;

    impl PtxUnparser for TxqTqueryB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "txq");
                    match &self.tquery {
                            Tquery::Width => {
                                    push_directive(tokens, "width");
                            }
                            Tquery::Height => {
                                    push_directive(tokens, "height");
                            }
                            Tquery::Depth => {
                                    push_directive(tokens, "depth");
                            }
                            Tquery::ChannelDataType => {
                                    push_directive(tokens, "channel_data_type");
                            }
                            Tquery::ChannelOrder => {
                                    push_directive(tokens, "channel_order");
                            }
                            Tquery::NormalizedCoords => {
                                    push_directive(tokens, "normalized_coords");
                            }
                            Tquery::ArraySize => {
                                    push_directive(tokens, "array_size");
                            }
                            Tquery::NumMipmapLevels => {
                                    push_directive(tokens, "num_mipmap_levels");
                            }
                            Tquery::NumSamples => {
                                    push_directive(tokens, "num_samples");
                            }
                    }
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TxqLevelTlqueryB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "txq");
                    push_directive(tokens, "level");
                    match &self.tlquery {
                            Tlquery::Width => {
                                    push_directive(tokens, "width");
                            }
                            Tlquery::Height => {
                                    push_directive(tokens, "height");
                            }
                            Tlquery::Depth => {
                                    push_directive(tokens, "depth");
                            }
                    }
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.lod.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TxqSqueryB32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "txq");
                    match &self.squery {
                            Squery::ForceUnnormalizedCoords => {
                                    push_directive(tokens, "force_unnormalized_coords");
                            }
                            Squery::FilterMode => {
                                    push_directive(tokens, "filter_mode");
                            }
                            Squery::AddrMode0 => {
                                    push_directive(tokens, "addr_mode_0");
                            }
                            Squery::AddrMode1 => {
                                    push_token_from_str(tokens, "addr_mode_1");
                            }
                            Squery::AddrMode2 => {
                                    push_token_from_str(tokens, "addr_mode_2");
                            }
                    }
                    push_directive(tokens, "b32");
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

