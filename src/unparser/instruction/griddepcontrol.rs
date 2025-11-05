//! Original PTX specification:
//!
//! griddepcontrol.action;
//! .action   = { .launch_dependents, .wait };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::griddepcontrol::section_0::*;

    impl PtxUnparser for GriddepcontrolAction {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "griddepcontrol");
                    match &self.action {
                            Action::LaunchDependents => {
                                    push_directive(tokens, "launch_dependents");
                            }
                            Action::Wait => {
                                    push_directive(tokens, "wait");
                            }
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

