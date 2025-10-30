use crate::{
    lexer::PtxToken,
    r#type::{
        common::RegisterOperand,
        instruction::vmad::{ComponentSelect, DataType, PlusOne, Scale, Standard, Vmad},
    },
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for Scale {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Scale::Shr7 => "shr7",
            Scale::Shr15 => "shr15",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

impl PtxUnparser for ComponentSelect {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ComponentSelect::B0 => "b0",
            ComponentSelect::B1 => "b1",
            ComponentSelect::B2 => "b2",
            ComponentSelect::B3 => "b3",
            ComponentSelect::H0 => "h0",
            ComponentSelect::H1 => "h1",
        };
        tokens.push(PtxToken::Directive(directive.to_string()));
    }
}

fn push_instruction_prefix(
    dtype: &DataType,
    atype: &DataType,
    btype: &DataType,
    extra_directive: Option<&str>,
    saturate: bool,
    scale: &Option<Scale>,
    tokens: &mut Vec<PtxToken>,
) {
    tokens.push(PtxToken::Identifier("vmad".to_string()));
    dtype.unparse_tokens(tokens);
    atype.unparse_tokens(tokens);
    btype.unparse_tokens(tokens);
    if let Some(name) = extra_directive {
        tokens.push(PtxToken::Directive(name.to_string()));
    }
    if saturate {
        tokens.push(PtxToken::Directive("sat".to_string()));
    }
    if let Some(scale) = scale {
        scale.unparse_tokens(tokens);
    }
}

fn push_selected_operand(
    negated: bool,
    register: &RegisterOperand,
    select: Option<&ComponentSelect>,
    tokens: &mut Vec<PtxToken>,
) {
    if negated {
        tokens.push(PtxToken::Minus);
    }
    register.unparse_tokens(tokens);
    if let Some(select) = select {
        select.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Standard {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_instruction_prefix(
            &self.dtype,
            &self.atype,
            &self.btype,
            None,
            self.saturate,
            &self.scale,
            tokens,
        );
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        push_selected_operand(self.a_negated, &self.a, self.asel.as_ref(), tokens);
        tokens.push(PtxToken::Comma);
        push_selected_operand(self.b_negated, &self.b, self.bsel.as_ref(), tokens);
        tokens.push(PtxToken::Comma);
        push_selected_operand(self.c_negated, &self.c, None, tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for PlusOne {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_instruction_prefix(
            &self.dtype,
            &self.atype,
            &self.btype,
            Some("po"),
            self.saturate,
            &self.scale,
            tokens,
        );
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        push_selected_operand(false, &self.a, self.asel.as_ref(), tokens);
        tokens.push(PtxToken::Comma);
        push_selected_operand(false, &self.b, self.bsel.as_ref(), tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Vmad {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vmad::Standard(standard) => standard.unparse_tokens(tokens),
            Vmad::PlusOne(plus_one) => plus_one.unparse_tokens(tokens),
        }
    }
}
