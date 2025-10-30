use crate::{lexer::PtxToken, r#type::instruction::mbarrier::*, unparser::*};

impl PtxUnparser for SharedSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            SharedSpace::Generic => {}
            SharedSpace::Shared => push_directive(tokens, "shared"),
            SharedSpace::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DataType::B64 => push_directive(tokens, "b64"),
        }
    }
}

impl PtxUnparser for Init {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "mbarrier");
        push_directive(tokens, "init");
        self.shared_space.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.count.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Mbarrier {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Mbarrier::Init(inst) => inst.unparse_tokens(tokens),
        }
    }
}
