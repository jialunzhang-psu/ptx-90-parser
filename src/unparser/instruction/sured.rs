//! Original PTX specification:
//!
//! sured.b.op.geom.ctype.mode [a,b],c; // byte addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .u32, .u64, .s32, .b32, .s64 };  // for sured.b
//! .mode  = { .trap, .clamp, .zero };
//! ----------------------------------------------------
//! sured.p.op.geom.ctype.mode [a,b],c; // sample addressing
//! .op    = { .add, .min, .max, .and, .or };
//! .geom  = { .1d, .2d, .3d };
//! .ctype = { .b32, .b64 };                    // for sured.p
//! .mode  = { .trap, .clamp, .zero };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::sured::section_0::*;

    impl PtxUnparser for SuredBOpGeomCtypeMode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "sured");
                    push_directive(tokens, "b");
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
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
                    }
                    match &self.ctype {
                            Ctype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Ctype::U64 => {
                                    push_directive(tokens, "u64");
                            }
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Ctype::S64 => {
                                    push_directive(tokens, "s64");
                            }
                    }
                    match &self.mode {
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
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

pub mod section_1 {
    use super::*;
    use crate::r#type::instruction::sured::section_1::*;

    impl PtxUnparser for SuredPOpGeomCtypeMode {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "sured");
                    push_directive(tokens, "p");
                    match &self.op {
                            Op::Add => {
                                    push_directive(tokens, "add");
                            }
                            Op::Min => {
                                    push_directive(tokens, "min");
                            }
                            Op::Max => {
                                    push_directive(tokens, "max");
                            }
                            Op::And => {
                                    push_directive(tokens, "and");
                            }
                            Op::Or => {
                                    push_directive(tokens, "or");
                            }
                    }
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
                    }
                    match &self.ctype {
                            Ctype::B32 => {
                                    push_directive(tokens, "b32");
                            }
                            Ctype::B64 => {
                                    push_directive(tokens, "b64");
                            }
                    }
                    match &self.mode {
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
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.c.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

