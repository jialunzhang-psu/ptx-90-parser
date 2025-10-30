use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::wmma::*},
    unparser::*,
};

fn push_wmma_load_prefix(tokens: &mut Vec<PtxToken>, variant: &str) {
    push_identifier(tokens, "wmma");
    push_directive(tokens, "load");
    push_directive(tokens, variant);
    push_directive(tokens, "sync");
    push_directive(tokens, "aligned");
}

fn push_layout_and_shape(layout: &Layout, shape: &Shape, tokens: &mut Vec<PtxToken>) {
    layout.unparse_tokens(tokens);
    shape.unparse_tokens(tokens);
}

fn push_optional_state_space(state_space: &Option<StateSpace>, tokens: &mut Vec<PtxToken>) {
    if let Some(space) = state_space {
        space.unparse_tokens(tokens);
    }
}

fn push_destination_and_address(
    destination: &RegisterOperand,
    address: &AddressOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    address.unparse_tokens(tokens);
}

impl PtxUnparser for Layout {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Layout::Row => "row",
            Layout::Col => "col",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Shape::M16N16K16 => "m16n16k16",
            Shape::M8N32K16 => "m8n32k16",
            Shape::M32N8K16 => "m32n8k16",
            Shape::M16N16K8 => "m16n16k8",
            Shape::M8N8K4 => "m8n8k4",
            Shape::M8N8K32 => "m8n8k32",
            Shape::M8N8K128 => "m8n8k128",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StateSpace::Global => push_directive(tokens, "global"),
            StateSpace::Shared => push_directive(tokens, "shared"),
            StateSpace::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for AType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            AType::F16 => "f16",
            AType::S8 => "s8",
            AType::U8 => "u8",
            AType::Bf16 => "bf16",
            AType::Tf32 => "tf32",
            AType::F64 => "f64",
            AType::S4 => "s4",
            AType::U4 => "u4",
            AType::B1 => "b1",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for BType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            BType::F16 => "f16",
            BType::S8 => "s8",
            BType::U8 => "u8",
            BType::Bf16 => "bf16",
            BType::Tf32 => "tf32",
            BType::F64 => "f64",
            BType::S4 => "s4",
            BType::U4 => "u4",
            BType::B1 => "b1",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for CType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            CType::F16 => "f16",
            CType::F32 => "f32",
            CType::S32 => "s32",
            CType::F64 => "f64",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for LoadA {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wmma_load_prefix(tokens, "a");
        push_layout_and_shape(&self.layout, &self.shape, tokens);
        push_optional_state_space(&self.state_space, tokens);
        self.data_type.unparse_tokens(tokens);
        push_destination_and_address(&self.destination, &self.address, tokens);
        if let Some(stride) = &self.stride {
            tokens.push(PtxToken::Comma);
            stride.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for LoadB {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wmma_load_prefix(tokens, "b");
        push_layout_and_shape(&self.layout, &self.shape, tokens);
        push_optional_state_space(&self.state_space, tokens);
        self.data_type.unparse_tokens(tokens);
        push_destination_and_address(&self.destination, &self.address, tokens);
        if let Some(stride) = &self.stride {
            tokens.push(PtxToken::Comma);
            stride.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for LoadC {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wmma_load_prefix(tokens, "c");
        push_layout_and_shape(&self.layout, &self.shape, tokens);
        push_optional_state_space(&self.state_space, tokens);
        self.data_type.unparse_tokens(tokens);
        push_destination_and_address(&self.destination, &self.address, tokens);
        if let Some(stride) = &self.stride {
            tokens.push(PtxToken::Comma);
            stride.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Instruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Instruction::LoadA(inner) => inner.unparse_tokens(tokens),
            Instruction::LoadB(inner) => inner.unparse_tokens(tokens),
            Instruction::LoadC(inner) => inner.unparse_tokens(tokens),
        }
    }
}
