use crate::{
    lexer::PtxToken,
    r#type::instruction::stmatrix::{DataType, Num, Shape, Source, StateSpace, Stmatrix},
    unparser::*,
};

impl PtxUnparser for Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Shape::M8N8 => "m8n8",
            Shape::M16N8 => "m16n8",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Num {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Num::X1 => "x1",
            Num::X2 => "x2",
            Num::X4 => "x4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::Shared => push_directive(tokens, "shared"),
            StateSpace::SharedCta => {
                push_directive(tokens, "shared");
                tokens.push(PtxToken::Colon);
                tokens.push(PtxToken::Colon);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::B16 => "b16",
            DataType::B8 => "b8",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Source {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBrace);
        match self {
            Source::X1(register) => register.unparse_tokens(tokens),
            Source::X2(registers) => {
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
            }
            Source::X4(registers) => {
                for (index, register) in registers.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    register.unparse_tokens(tokens);
                }
            }
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for Stmatrix {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "stmatrix");
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
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
