use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::vset4::*},
    unparser::*,
};

fn lane_to_char(lane: Lane) -> char {
    match lane {
        Lane::B0 => '0',
        Lane::B1 => '1',
        Lane::B2 => '2',
        Lane::B3 => '3',
        Lane::B4 => '4',
        Lane::B5 => '5',
        Lane::B6 => '6',
        Lane::B7 => '7',
    }
}

impl PtxUnparser for OperandType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = match self {
            OperandType::U32 => "u32",
            OperandType::S32 => "s32",
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
            Mask::B0 => "b0",
            Mask::B1 => "b1",
            Mask::B10 => "b10",
            Mask::B2 => "b2",
            Mask::B20 => "b20",
            Mask::B21 => "b21",
            Mask::B210 => "b210",
            Mask::B3 => "b3",
            Mask::B30 => "b30",
            Mask::B31 => "b31",
            Mask::B310 => "b310",
            Mask::B32 => "b32",
            Mask::B320 => "b320",
            Mask::B321 => "b321",
            Mask::B3210 => "b3210",
        };
        push_directive(tokens, suffix);
    }
}

impl PtxUnparser for Selector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let suffix = format!(
            "b{}{}{}{}",
            lane_to_char(self.x),
            lane_to_char(self.y),
            lane_to_char(self.z),
            lane_to_char(self.w)
        );
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

impl PtxUnparser for SourceA {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(selector) = &self.selector {
            selector.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for SourceB {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.register.unparse_tokens(tokens);
        if let Some(selector) = &self.selector {
            selector.unparse_tokens(tokens);
        }
    }
}

fn push_prefix(
    a_type: &OperandType,
    b_type: &OperandType,
    compare_op: &CompareOp,
    tokens: &mut Vec<PtxToken>,
) {
    tokens.push(PtxToken::Identifier("vset4".to_string()));
    a_type.unparse_tokens(tokens);
    b_type.unparse_tokens(tokens);
    compare_op.unparse_tokens(tokens);
}

fn push_operands(
    destination: &Destination,
    a: &SourceA,
    b: &SourceB,
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

impl PtxUnparser for Vset4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vset4::SimdMerge {
                a_type,
                b_type,
                compare_op,
                destination,
                a,
                b,
                c,
            } => {
                push_prefix(a_type, b_type, compare_op, tokens);
                push_operands(destination, a, b, c, tokens);
            }
            Vset4::Accumulate {
                a_type,
                b_type,
                compare_op,
                destination,
                a,
                b,
                c,
            } => {
                push_prefix(a_type, b_type, compare_op, tokens);
                push_directive(tokens, "add");
                push_operands(destination, a, b, c, tokens);
            }
        }
    }
}
