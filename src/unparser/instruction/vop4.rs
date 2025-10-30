use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::vop4::*},
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

impl PtxUnparser for Operation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let opcode = match self {
            Operation::Vadd4 => "vadd4",
            Operation::Vsub4 => "vsub4",
            Operation::Vavrg4 => "vavrg4",
            Operation::Vabsdiff4 => "vabsdiff4",
            Operation::Vmin4 => "vmin4",
            Operation::Vmax4 => "vmax4",
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
        tokens.push(PtxToken::Directive(suffix.to_string()));
    }
}

impl PtxUnparser for Selector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let lanes = self.lanes;
        let suffix = format!(
            "b{}{}{}{}",
            lane_to_char(lanes[0]),
            lane_to_char(lanes[1]),
            lane_to_char(lanes[2]),
            lane_to_char(lanes[3]),
        );
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
            &self.data_type,
            &self.a_type,
            &self.b_type,
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
            &self.data_type,
            &self.a_type,
            &self.b_type,
            tokens,
        );
        tokens.push(PtxToken::Directive("add".to_string()));
        push_operands(&self.destination, &self.a, &self.b, &self.c, tokens);
    }
}

impl PtxUnparser for Vop4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vop4::Merge(merge) => merge.unparse_tokens(tokens),
            Vop4::Accumulate(accumulate) => accumulate.unparse_tokens(tokens),
        }
    }
}
