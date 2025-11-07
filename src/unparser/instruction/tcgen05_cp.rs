//! Original PTX specification:
//!
//! tcgen05.cp.cta_group.shape{.multicast}{.dst_src_fmt} [taddr], s-desc;
//! .cta_group = { .cta_group::1, .cta_group::2 };
//! .dst_src_fmt   = { .b8x16.b6x16_p32, .b8x16.b4x16_p64 };
//! .shape     = { .128x256b, .4x256b, .128x128b, .64x128b**, .32x128b*** };
//! .multicast = { .warpx2::02_13** , .warpx2::01_23**, .warpx4*** };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_cp::section_0::*;

    impl PtxUnparser for Tcgen05CpCtaGroupShapeMulticastDstSrcFmt {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "cp");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    match &self.shape {
                            Shape::_32x128b => {
                                    push_directive(tokens, "32x128b***");
                            }
                            Shape::_64x128b => {
                                    push_directive(tokens, "64x128b**");
                            }
                            Shape::_128x256b => {
                                    push_directive(tokens, "128x256b");
                            }
                            Shape::_128x128b => {
                                    push_directive(tokens, "128x128b");
                            }
                            Shape::_4x256b => {
                                    push_directive(tokens, "4x256b");
                            }
                    }
                    if let Some(multicast_0) = self.multicast.as_ref() {
                            match multicast_0 {
                                    Multicast::Warpx20213 => {
                                            push_directive(tokens, "warpx2::02_13**");
                                    }
                                    Multicast::Warpx20123 => {
                                            push_directive(tokens, "warpx2::01_23**");
                                    }
                                    Multicast::Warpx4 => {
                                            push_directive(tokens, "warpx4***");
                                    }
                            }
                    }
                    if let Some(dst_src_fmt_1) = self.dst_src_fmt.as_ref() {
                            match dst_src_fmt_1 {
                                    DstSrcFmt::B8x16B6x16P32 => {
                                            push_directive(tokens, "b8x16.b6x16_p32");
                                    }
                                    DstSrcFmt::B8x16B4x16P64 => {
                                            push_directive(tokens, "b8x16.b4x16_p64");
                                    }
                            }
                    }
                    self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.s_desc.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

