//! Original PTX specification:
//!
//! // direct call to named function, func is a symbol
//! call{.uni} (ret-param), func, (param-list);
//! call{.uni} func, (param-list);
//! call{.uni} func;
//! // indirect call via pointer, with full list of call targets
//! call{.uni} (ret-param), fptr, (param-list), flist;
//! call{.uni} fptr, (param-list), flist;
//! call{.uni} fptr, flist;
//! // indirect call via pointer, with no knowledge of call targets
//! call{.uni} (ret-param), fptr, (param-list), fproto;
//! call{.uni} fptr, (param-list), fproto;
//! call{.uni} fptr, fproto;

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::call::section_0::*;

    impl PtxUnparser for CallUni {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    tokens.push(PtxToken::LParen);
                    self.ret_param.unparse_tokens(tokens);
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.func.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.func.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni2 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.func.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni3 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    tokens.push(PtxToken::LParen);
                    self.ret_param.unparse_tokens(tokens);
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.flist.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni4 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.flist.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni5 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.flist.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni6 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    tokens.push(PtxToken::LParen);
                    self.ret_param.unparse_tokens(tokens);
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.fproto.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni7 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LParen);
                    for (idx, operand) in self.param_list.iter().enumerate() {
                        if idx > 0 { tokens.push(PtxToken::Comma); }
                        operand.unparse_tokens(tokens);
                    }
                    tokens.push(PtxToken::RParen);
            tokens.push(PtxToken::Comma);
                    self.fproto.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for CallUni8 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "call");
                    if self.uni {
                            push_directive(tokens, "uni");
                    }
                    self.fptr.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.fproto.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

