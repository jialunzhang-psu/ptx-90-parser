//! Original PTX specification:
//!
//! tcgen05.shift.cta_group.down  [taddr];
//! .cta_group = { .cta_group::1, .cta_group::2 }

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tcgen05_shift::section_0::*;

    impl PtxUnparser for Tcgen05ShiftCtaGroupDown {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tcgen05");
                    push_directive(tokens, "shift");
                    match &self.cta_group {
                            CtaGroup::CtaGroup1 => {
                                    push_directive(tokens, "cta_group::1");
                            }
                            CtaGroup::CtaGroup2 => {
                                    push_directive(tokens, "cta_group::2");
                            }
                    }
                    push_directive(tokens, "down");
                    self.taddr.unparse_tokens(tokens);
            tokens.push(PtxToken::Semicolon);
        }
    }

}

