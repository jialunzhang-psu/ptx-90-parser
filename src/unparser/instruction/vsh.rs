use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vsh::*},
    unparser::*,
};

fn push_common_prefix(
    opcode: &Opcode,
    dtype: &DataType,
    atype: &DataType,
    saturate: bool,
    mode: &Mode,
    tokens: &mut Vec<PtxToken>,
) {
    opcode.unparse_tokens(tokens);
    dtype.unparse_tokens(tokens);
    atype.unparse_tokens(tokens);
    tokens.push(PtxToken::Directive("u32".to_string()));
    if saturate {
        tokens.push(PtxToken::Directive("sat".to_string()));
    }
    mode.unparse_tokens(tokens);
}

fn push_scalar_operands(
    destination: &RegisterOperand,
    a: &Source,
    b: &Source,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    b.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

fn push_scalar_with_secondary_operands(
    destination: &RegisterOperand,
    a: &Source,
    b: &Source,
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

fn push_merge_operands(
    destination: &MergeDestination,
    a: &Source,
    b: &Source,
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

impl PtxUnparser for Opcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let opcode = match self {
            Opcode::Vshl => "vshl",
            Opcode::Vshr => "vshr",
        };
        tokens.push(PtxToken::Identifier(opcode.to_string()));
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            Mode::Clamp => "clamp",
            Mode::Wrap => "wrap",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Selection {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            Selection::B0 => "b0",
            Selection::B1 => "b1",
            Selection::B2 => "b2",
            Selection::B3 => "b3",
            Selection::H0 => "h0",
            Selection::H1 => "h1",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for SecondaryOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            SecondaryOp::Add => "add",
            SecondaryOp::Min => "min",
            SecondaryOp::Max => "max",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Source {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(selection) = &self.selection {
            selection.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for MergeDestination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        self.selection.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Scalar {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(
            &self.opcode,
            &self.dtype,
            &self.atype,
            self.saturate,
            &self.mode,
            tokens,
        );
        push_scalar_operands(&self.destination, &self.a, &self.b, tokens);
    }
}

impl PtxUnparser for ScalarWithSecondary {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(
            &self.opcode,
            &self.dtype,
            &self.atype,
            self.saturate,
            &self.mode,
            tokens,
        );
        self.secondary.unparse_tokens(tokens);
        push_scalar_with_secondary_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for DataMerge {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_prefix(
            &self.opcode,
            &self.dtype,
            &self.atype,
            self.saturate,
            &self.mode,
            tokens,
        );
        push_merge_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Vsh {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vsh::Scalar(scalar) => scalar.unparse_tokens(tokens),
            Vsh::ScalarWithSecondary(with_secondary) => with_secondary.unparse_tokens(tokens),
            Vsh::DataMerge(merge) => merge.unparse_tokens(tokens),
        }
    }
}
