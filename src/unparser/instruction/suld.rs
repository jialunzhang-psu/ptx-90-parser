//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype.mode [a, b];  // unformatted
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d };
//! .cop   = { .ca, .cg, .cs, .cv };               // cache operation
//! .vec   = { none, .v2, .v4 };
//! .dtype = { .b8 , .b16, .b32, .b64 };
//! .mode = { .trap, .clamp, .zero };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::suld::section_0::*;

    impl PtxUnparser for SuldBGeomCopVecDtypeMode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "suld");
                    push_directive(tokens, "b");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                    }
                    if let Some(cop_0) = self.cop.as_ref() {
                            match cop_0 {
                                    Cop::Ca => {
                                            push_directive(tokens, "ca");
                                    }
                                    Cop::Cg => {
                                            push_directive(tokens, "cg");
                                    }
                                    Cop::Cs => {
                                            push_directive(tokens, "cs");
                                    }
                                    Cop::Cv => {
                                            push_directive(tokens, "cv");
                                    }
                            }
                    }
                    match &self.vec {
                            Vec::None => {
                                    push_token_from_str(tokens, "none");
                            }
                            Vec::V2 => {
                                    push_directive(tokens, "v2");
                            }
                            Vec::V4 => {
                                    push_directive(tokens, "v4");
                            }
                    }
                    match &self.dtype {
                            Dtype::B8 => {
                                    push_directive(tokens, "b8");
                            }
                            Dtype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Dtype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Dtype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    match &self.mode {
                            Mode::Trap => {
                                    push_directive(tokens, "trap");
                            }
                            Mode::Clamp => {
                                    push_directive(tokens, "clamp");
                            }
                            Mode::Zero => {
                                    push_directive(tokens, "zero");
                            }
                    }
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_1_0, ref group_1_1) = &self.a;
                    group_1_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_1_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

