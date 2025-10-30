use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::tld4::*},
    unparser::*,
};

fn geometry_parts<'a>(
    geometry: &'a Geometry,
) -> (
    &'static str,
    &'a RegisterOperand,
    Option<&'a RegisterOperand>,
) {
    match geometry {
        Geometry::TwoD {
            coordinates,
            offset,
        } => ("2d", coordinates, offset.as_ref()),
        Geometry::Array2D {
            coordinates,
            offset,
        } => ("a2d", coordinates, offset.as_ref()),
        Geometry::Cube { coordinates } => ("cube", coordinates, None),
        Geometry::ArrayCube { coordinates } => ("acube", coordinates, None),
    }
}

fn unparse_instruction_body(
    tokens: &mut Vec<PtxToken>,
    component: Component,
    geometry: &Geometry,
    data_type: DataType,
    destination: &Destination,
    texture: &TextureOperand,
    sampler: Option<&SamplerOperand>,
    depth_compare: Option<&RegisterOperand>,
) {
    component.unparse_tokens(tokens);

    let (modifier, coordinates, offset) = geometry_parts(geometry);
    push_directive(tokens, modifier);

    push_directive(tokens, "v4");
    push_directive(tokens, "dtype");
    data_type.unparse_tokens(tokens);

    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    tokens.push(PtxToken::LBracket);
    texture.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    if let Some(sampler) = sampler {
        sampler.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
    }
    coordinates.unparse_tokens(tokens);
    tokens.push(PtxToken::RBracket);

    match offset {
        Some(offset) => {
            tokens.push(PtxToken::Comma);
            offset.unparse_tokens(tokens);
            if let Some(depth_compare) = depth_compare {
                tokens.push(PtxToken::Comma);
                depth_compare.unparse_tokens(tokens);
            }
        }
        None => {
            if let Some(depth_compare) = depth_compare {
                tokens.push(PtxToken::Comma);
                depth_compare.unparse_tokens(tokens);
            }
        }
    }

    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Component {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Component::R => "r",
            Component::G => "g",
            Component::B => "b",
            Component::A => "a",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            DataType::U32 => "u32",
            DataType::S32 => "s32",
            DataType::F32 => "f32",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.vector.unparse_tokens(tokens);
        if let Some(predicate) = &self.predicate {
            tokens.push(PtxToken::Pipe);
            predicate.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for TextureOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            TextureOperand::Symbol(symbol) => symbol.unparse_tokens(tokens),
            TextureOperand::Register(register) => register.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for SamplerOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            SamplerOperand::Symbol(symbol) => symbol.unparse_tokens(tokens),
            SamplerOperand::Register(register) => register.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Tld4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "tld4");
        match self {
            Tld4::Implicit(instr) => unparse_instruction_body(
                tokens,
                instr.component,
                &instr.geometry,
                instr.data_type,
                &instr.destination,
                &instr.texture,
                None,
                instr.depth_compare.as_ref(),
            ),
            Tld4::Explicit(instr) => unparse_instruction_body(
                tokens,
                instr.component,
                &instr.geometry,
                instr.data_type,
                &instr.destination,
                &instr.texture,
                Some(&instr.sampler),
                instr.depth_compare.as_ref(),
            ),
        }
    }
}
