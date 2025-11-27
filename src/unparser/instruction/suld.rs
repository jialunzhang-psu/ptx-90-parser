//! Original PTX specification:
//!
//! suld.b.geom{.cop}.vec.dtype{.mode}  d, [a, b];  // unformatted
//! 
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
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "suld");
                    push_directive(tokens, "b");
                    match &self.geom {
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                            Dtype::B16 => {
                                    push_directive(tokens, "b16");
                            }
                            Dtype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Dtype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                            Dtype::B8 => {
                                    push_directive(tokens, "b8");
                            }
                    }
                    if let Some(mode_1) = self.mode.as_ref() {
                            match mode_1 {
                                    Mode::Clamp => {
                                            push_directive(tokens, "clamp");
                                    }
                                    Mode::Trap => {
                                            push_directive(tokens, "trap");
                                    }
                                    Mode::Zero => {
                                            push_directive(tokens, "zero");
                                    }
                            }
                    }
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

