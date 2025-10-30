use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vset2::*},
    unparser::*,
};

fn half_to_char(half: Half) -> char {
    match half {
        Half::H0 => '0',
        Half::H1 => '1',
        Half::H2 => '2',
        Half::H3 => '3',
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
        };
        push_directive(tokens, suffix);
    }
}

impl PtxUnparser for CompareOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            CompareOp::Eq => "eq",
            CompareOp::Ne => "ne",
            CompareOp::Lt => "lt",
            CompareOp::Le => "le",
            CompareOp::Gt => "gt",
            CompareOp::Ge => "ge",
        };
        push_directive(tokens, suffix);
    }
}

impl PtxUnparser for Mask {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            Mask::H0 => "h0",
            Mask::H1 => "h1",
            Mask::H10 => "h10",
        };
        push_directive(tokens, suffix);
    }
}

impl PtxUnparser for Selector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let halves = self.halves;
        let suffix = format!("h{}{}", half_to_char(halves[0]), half_to_char(halves[1]));
        push_directive(tokens, &suffix);
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(mask) = &self.mask {
            mask.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for ASource {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(selector) = &self.selector {
            selector.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for BSource {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(selector) = &self.selector {
            selector.unparse_tokens(tokens);
        }
    }
}

fn push_prefix(
    a_type: &DataType,
    b_type: &DataType,
    comparison: &CompareOp,
    tokens: &mut Vec<PtxToken>,
) {
    tokens.push(PtxToken::Identifier("vset2".to_string()));
    a_type.unparse_tokens(tokens);
    b_type.unparse_tokens(tokens);
    comparison.unparse_tokens(tokens);
}

fn push_operands(
    destination: &Destination,
    a: &ASource,
    b: &BSource,
    c: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    c.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for SimdMerge {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_prefix(&self.a_type, &self.b_type, &self.comparison, tokens);
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Accumulate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_prefix(&self.a_type, &self.b_type, &self.comparison, tokens);
        push_directive(tokens, "add");
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Vset2 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vset2::SimdMerge(instruction) => instruction.unparse_tokens(tokens),
            Vset2::Accumulate(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
