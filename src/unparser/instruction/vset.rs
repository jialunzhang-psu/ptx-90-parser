use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vset::*},
    unparser::*,
};

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for Compare {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Compare::Eq => "eq",
            Compare::Ne => "ne",
            Compare::Lt => "lt",
            Compare::Le => "le",
            Compare::Gt => "gt",
            Compare::Ge => "ge",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for Selection {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            Selection::B0 => "b0",
            Selection::B1 => "b1",
            Selection::B2 => "b2",
            Selection::B3 => "b3",
            Selection::H0 => "h0",
            Selection::H1 => "h1",
        };
        push_directive(tokens, name);
    }
}

impl PtxUnparser for SecondaryOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            SecondaryOperation::Add => "add",
            SecondaryOperation::Min => "min",
            SecondaryOperation::Max => "max",
        };
        push_directive(tokens, name);
    }
}

fn push_register_with_selection(
    register: &RegisterOperand,
    selection: Option<&Selection>,
    tokens: &mut Vec<PtxToken>,
) {
    register.unparse_tokens(tokens);
    if let Some(selection) = selection {
        selection.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Source {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_register_with_selection(&self.register, self.selection.as_ref(), tokens);
    }
}

fn push_prefix(
    instruction: &str,
    atype: &DataType,
    btype: &DataType,
    cmp: &Compare,
    tokens: &mut Vec<PtxToken>,
) {
    tokens.push(PtxToken::Identifier(instruction.to_string()));
    atype.unparse_tokens(tokens);
    btype.unparse_tokens(tokens);
    cmp.unparse_tokens(tokens);
}

impl PtxUnparser for Scalar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_prefix("vset", &self.atype, &self.btype, &self.cmp, tokens);
        self.d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ScalarWithSecondary {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_prefix("vset", &self.atype, &self.btype, &self.cmp, tokens);
        self.op2.unparse_tokens(tokens);
        self.d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for DataMerge {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_prefix("vset", &self.atype, &self.btype, &self.cmp, tokens);
        push_register_with_selection(&self.d, Some(&self.dsel), tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.c.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Vset {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vset::Scalar(instruction) => instruction.unparse_tokens(tokens),
            Vset::ScalarWithSecondary(instruction) => instruction.unparse_tokens(tokens),
            Vset::DataMerge(instruction) => instruction.unparse_tokens(tokens),
        }
    }
}
