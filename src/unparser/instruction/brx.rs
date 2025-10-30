use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::brx::*},
    unparser::*,
};

fn push_opcode(tokens: &mut Vec<PtxToken>) {
    tokens.push(PtxToken::Identifier("brx".to_string()));
    tokens.push(PtxToken::Directive("idx".to_string()));
}

fn push_optional_uniform(tokens: &mut Vec<PtxToken>, uniform: bool) {
    if uniform {
        tokens.push(PtxToken::Directive("uni".to_string()));
    }
}

fn push_index_operand(index: &RegisterOperand, tokens: &mut Vec<PtxToken>) {
    index.unparse_tokens(tokens);
}

fn push_target_list(targets: &Label, tokens: &mut Vec<PtxToken>) {
    targets.unparse_tokens(tokens);
}

impl PtxUnparser for Brx {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens);
        push_optional_uniform(tokens, self.uniform);
        push_index_operand(&self.index, tokens);
        tokens.push(PtxToken::Comma);
        push_target_list(&self.targets, tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
