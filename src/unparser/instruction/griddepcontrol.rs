#![allow(unused_imports)]

use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::griddepcontrol::*},
    unparser::*,
};

impl PtxUnparser for Griddepcontrol {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "griddepcontrol");
        let modifier = match self {
            Griddepcontrol::LaunchDependents => "launch_dependents",
            Griddepcontrol::Wait => "wait",
        };
        push_directive(tokens, modifier);
        tokens.push(PtxToken::Semicolon);
    }
}
