use crate::{
    lexer::PtxToken,
    r#type::instruction::suq::{Operand, Query, Suq},
    unparser::*,
};

impl PtxUnparser for Query {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Query::Width => push_directive(tokens, "width"),
            Query::Height => push_directive(tokens, "height"),
            Query::Depth => push_directive(tokens, "depth"),
            Query::ChannelDataType => push_directive(tokens, "channel_data_type"),
            Query::ChannelOrder => push_directive(tokens, "channel_order"),
            Query::ArraySize => push_directive(tokens, "array_size"),
            Query::MemoryLayout => push_directive(tokens, "memory_layout"),
        }
    }
}

impl PtxUnparser for Operand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBracket);
        match self {
            Operand::Surface(symbol) => symbol.unparse_tokens(tokens),
            Operand::Register(register) => register.unparse_tokens(tokens),
        }
        tokens.push(PtxToken::RBracket);
    }
}

impl PtxUnparser for Suq {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "suq");
        self.query.unparse_tokens(tokens);
        push_directive(tokens, "b32");
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
