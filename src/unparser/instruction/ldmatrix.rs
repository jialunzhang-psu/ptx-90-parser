use crate::{lexer::PtxToken, r#type::instruction::ldmatrix, unparser::*};

impl PtxUnparser for ldmatrix::Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ldmatrix::Shape::M8N8 => "m8n8",
            ldmatrix::Shape::M16N16 => "m16n16",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ldmatrix::Num {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ldmatrix::Num::X1 => "x1",
            ldmatrix::Num::X2 => "x2",
            ldmatrix::Num::X4 => "x4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ldmatrix::StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ldmatrix::StateSpace::Shared => push_directive(tokens, "shared"),
            ldmatrix::StateSpace::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for ldmatrix::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ldmatrix::DataType::B16 => "b16",
            ldmatrix::DataType::B8 => "b8",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ldmatrix::DestinationFormat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ldmatrix::DestinationFormat::B8x16 => push_directive(tokens, "b8x16"),
        }
    }
}

impl PtxUnparser for ldmatrix::SourceFormat {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ldmatrix::SourceFormat::B6x16P32 => "b6x16_p32",
            ldmatrix::SourceFormat::B4x16P64 => "b4x16_p64",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for ldmatrix::Standard {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ldmatrix");
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        self.shape.unparse_tokens(tokens);
        self.num.unparse_tokens(tokens);
        if self.trans {
            push_directive(tokens, "trans");
        }
        if let Some(state_space) = self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.data_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ldmatrix::M8N16 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ldmatrix");
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        push_directive(tokens, "m8n16");
        self.num.unparse_tokens(tokens);
        if let Some(state_space) = self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.destination_format.unparse_tokens(tokens);
        self.source_format.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ldmatrix::M16N16 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "ldmatrix");
        push_directive(tokens, "sync");
        push_directive(tokens, "aligned");
        push_directive(tokens, "m16n16");
        self.num.unparse_tokens(tokens);
        push_directive(tokens, "trans");
        if let Some(state_space) = self.state_space {
            state_space.unparse_tokens(tokens);
        }
        self.destination_format.unparse_tokens(tokens);
        self.source_format.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ldmatrix::Ldmatrix {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ldmatrix::Ldmatrix::Standard(standard) => standard.unparse_tokens(tokens),
            ldmatrix::Ldmatrix::M8N16(m8n16) => m8n16.unparse_tokens(tokens),
            ldmatrix::Ldmatrix::M16N16(m16n16) => m16n16.unparse_tokens(tokens),
        }
    }
}
