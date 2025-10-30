use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vop2::*},
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

impl PtxUnparser for Operation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let opcode = match self {
            Operation::Vadd2 => "vadd2",
            Operation::Vsub2 => "vsub2",
            Operation::Vavrg2 => "vavrg2",
            Operation::Vabsdiff2 => "vabsdiff2",
            Operation::Vmin2 => "vmin2",
            Operation::Vmax2 => "vmax2",
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

impl PtxUnparser for Mask {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            Mask::H0 => "h0",
            Mask::H1 => "h1",
            Mask::H10 => "h10",
        };
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Selector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let halves = self.halves;
        let suffix = format!("h{}{}", half_to_char(halves[0]), half_to_char(halves[1]));
        tokens.push(PtxToken::Directive(suffix));
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

fn push_common_suffixes(
    operation: &Operation,
    dtype: &DataType,
    atype: &DataType,
    btype: &DataType,
    tokens: &mut Vec<PtxToken>,
) {
    operation.unparse_tokens(tokens);
    dtype.unparse_tokens(tokens);
    atype.unparse_tokens(tokens);
    btype.unparse_tokens(tokens);
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

impl PtxUnparser for Merge {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_suffixes(
            &self.operation,
            &self.dtype,
            &self.atype,
            &self.btype,
            tokens,
        );
        if self.saturate {
            tokens.push(PtxToken::Directive("sat".to_string()));
        }
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Accumulate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_common_suffixes(
            &self.operation,
            &self.dtype,
            &self.atype,
            &self.btype,
            tokens,
        );
        tokens.push(PtxToken::Directive("add".to_string()));
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Vop2 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vop2::Merge(merge) => merge.unparse_tokens(tokens),
            Vop2::Accumulate(accumulate) => accumulate.unparse_tokens(tokens),
        }
    }
}
