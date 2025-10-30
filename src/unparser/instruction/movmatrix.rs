use crate::{lexer::PtxToken, r#type::instruction::movmatrix::*, unparser::*};

impl PtxUnparser for Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Shape::M8N8 => "m8n8",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::B16 => "b16",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Movmatrix {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "movmatrix");
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "trans");
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
