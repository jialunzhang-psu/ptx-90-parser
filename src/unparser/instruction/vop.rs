use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vop::*},
    unparser::*,
};

fn push_common_suffixes(
    opcode: &Opcode,
    dtype: &DataType,
    atype: &DataType,
    btype: &DataType,
    tokens: &mut Vec<PtxToken>,
) {
    opcode.unparse_tokens(tokens);
    dtype.unparse_tokens(tokens);
    atype.unparse_tokens(tokens);
    btype.unparse_tokens(tokens);
}

fn push_merge_operands(
    destination: &MergeDestination,
    a: &Operand,
    b: &Operand,
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
            Opcode::Vadd => "vadd",
            Opcode::Vsub => "vsub",
            Opcode::Vabsdiff => "vabsdiff",
            Opcode::Vmin => "vmin",
            Opcode::Vmax => "vmax",
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

impl PtxUnparser for SecondaryOpcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            SecondaryOpcode::Add => "add",
            SecondaryOpcode::Min => "min",
            SecondaryOpcode::Max => "max",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Operand {
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
        push_common_suffixes(&self.opcode, &self.dtype, &self.atype, &self.btype, tokens);
        if self.saturate {
            tokens.push(PtxToken::Directive("sat".to_string()));
        }
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
        push_common_suffixes(&self.opcode, &self.dtype, &self.atype, &self.btype, tokens);
        if self.saturate {
            tokens.push(PtxToken::Directive("sat".to_string()));
        }
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
        push_common_suffixes(&self.opcode, &self.dtype, &self.atype, &self.btype, tokens);
        if self.saturate {
            tokens.push(PtxToken::Directive("sat".to_string()));
        }
        push_merge_operands(&self.d, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Vop {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vop::Scalar(scalar) => scalar.unparse_tokens(tokens),
            Vop::ScalarWithSecondary(with_secondary) => with_secondary.unparse_tokens(tokens),
            Vop::DataMerge(merge) => merge.unparse_tokens(tokens),
        }
    }
}
