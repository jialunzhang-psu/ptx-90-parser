//! Original PTX specification:
//!
//! clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 pred, try_cancel_response;
//! clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;
//! clusterlaunchcontrol.query_cancel{.get_first_ctaid::dimension}.b32.b128 reg, try_cancel_response;
//! .get_first_ctaid::dimension = { .get_first_ctaid::x, .get_first_ctaid::y, .get_first_ctaid::z };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::*;

    impl PtxUnparser for ClusterlaunchcontrolQueryCancelIsCanceledPredB128 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "clusterlaunchcontrol");
            push_directive(tokens, "query_cancel");
            push_directive(tokens, "is_canceled");
            push_directive(tokens, "pred");
            push_directive(tokens, "b128");
            self.pred2.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.try_cancel_response.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "clusterlaunchcontrol");
            push_directive(tokens, "query_cancel");
            push_directive(tokens, "get_first_ctaid");
            push_directive(tokens, "v4");
            push_directive(tokens, "b32");
            push_directive(tokens, "b128");
            self.xdim.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.try_cancel_response.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "clusterlaunchcontrol");
            push_directive(tokens, "query_cancel");
            if let Some(get_first_ctaid_dimension_0) = self.get_first_ctaid_dimension.as_ref() {
                match get_first_ctaid_dimension_0 {
                    GetFirstCtaidDimension::GetFirstCtaidX => {
                        push_directive(tokens, "get_first_ctaid::x");
                    }
                    GetFirstCtaidDimension::GetFirstCtaidY => {
                        push_directive(tokens, "get_first_ctaid::y");
                    }
                    GetFirstCtaidDimension::GetFirstCtaidZ => {
                        push_directive(tokens, "get_first_ctaid::z");
                    }
                }
            }
            push_directive(tokens, "b32");
            push_directive(tokens, "b128");
            self.reg.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
            self.try_cancel_response.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }
}
