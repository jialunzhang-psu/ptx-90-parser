use crate::unparser::*;
use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::discard::*},
};
fn unparse_address(tokens: &mut Vec<PtxToken>, address: &AddressOperand) {
    address.unparse_tokens(tokens);
}

impl PtxUnparser for Space {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Space::Global => push_directive(tokens, "global"),
        }
    }
}

impl PtxUnparser for Level {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Level::L2 => push_directive(tokens, "L2"),
        }
    }
}

impl PtxUnparser for Size {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Size::Bytes128 => push_decimal(tokens, "128"),
        }
    }
}

impl PtxUnparser for Discard {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "discard");

        if let Some(space) = &self.space {
            space.unparse_tokens(tokens);
        }

        self.level.unparse_tokens(tokens);

        // Address operands already include their surrounding brackets.
        unparse_address(tokens, &self.address);

        tokens.push(PtxToken::Comma);
        self.size.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
