//! Original PTX specification:
//!
//! testp.op.type  p, a;  // result is .pred
//! .op   = { .finite, .infinite,
//! .number, .notanumber,
//! .normal, .subnormal };
//! .type = { .f32, .f64 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::testp::section_0::*;

    impl PtxUnparser for TestpOpType {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "testp");
                    match &self.op {
                            Op::Notanumber => {
                                    push_directive(tokens, "notanumber");
                            }
                            Op::Subnormal => {
                                    push_directive(tokens, "subnormal");
                            }
                            Op::Infinite => {
                                    push_directive(tokens, "infinite");
                            }
                            Op::Finite => {
                                    push_directive(tokens, "finite");
                            }
                            Op::Number => {
                                    push_directive(tokens, "number");
                            }
                            Op::Normal => {
                                    push_directive(tokens, "normal");
                            }
                    }
                    match &self.type_ {
                            Type::F32 => {
                                    push_directive(tokens, "f32");
                            }
                            Type::F64 => {
                                    push_directive(tokens, "f64");
                            }
                    }
                    self.p.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.a.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

