//! Original PTX specification:
//!
//! cp.async.mbarrier.arrive{.noinc}{.state}.b64 [addr];
//! .state = { .shared, .shared::cta}

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::cp_async_mbarrier_arrive::section_0::*;

    impl PtxUnparser for CpAsyncMbarrierArriveNoincStateB64 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "cp");
                    push_directive(tokens, "async");
                    push_directive(tokens, "mbarrier");
                    push_directive(tokens, "arrive");
                    if self.noinc {
                            push_directive(tokens, "noinc");
                    }
                    if let Some(state_0) = self.state.as_ref() {
                            match state_0 {
                                    State::Shared => {
                                            push_directive(tokens, "shared");
                                    }
                                    State::SharedCta => {
                                            push_directive(tokens, "shared::cta");
                                    }
                            }
                    }
                    push_directive(tokens, "b64");
                    self.addr.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

