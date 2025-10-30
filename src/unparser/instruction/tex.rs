use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::tex::*},
    unparser::*,
};

fn push_vector4_header(
    tokens: &mut Vec<PtxToken>,
    modifier: Option<&str>,
    geometry: Geometry,
    data_type: Vector4DataType,
    coordinate_type: CoordinateType,
    destination: &RegisterOperand,
    predicate: Option<&PredicateRegister>,
) {
    if let Some(modifier) = modifier {
        push_directive(tokens, modifier);
    }
    geometry.unparse_tokens(tokens);
    push_directive(tokens, "v4");
    data_type.unparse_tokens(tokens);
    coordinate_type.unparse_tokens(tokens);
    destination.unparse_tokens(tokens);
    if let Some(predicate) = predicate {
        tokens.push(PtxToken::Pipe);
        predicate.unparse_tokens(tokens);
    }
}

fn push_vector2_header(
    tokens: &mut Vec<PtxToken>,
    modifier: Option<&str>,
    geometry: Geometry,
    coordinate_type: CoordinateType,
    destination: &RegisterOperand,
    predicate: Option<&PredicateRegister>,
) {
    if let Some(modifier) = modifier {
        push_directive(tokens, modifier);
    }
    geometry.unparse_tokens(tokens);
    push_directive(tokens, "v2");
    push_directive(tokens, "f16x2");
    coordinate_type.unparse_tokens(tokens);
    destination.unparse_tokens(tokens);
    if let Some(predicate) = predicate {
        tokens.push(PtxToken::Pipe);
        predicate.unparse_tokens(tokens);
    }
}

fn push_texture_operands(
    tokens: &mut Vec<PtxToken>,
    texture: &TextureOperand,
    sampler: Option<&SamplerOperand>,
    coordinates: &RegisterOperand,
) {
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
}

fn push_optional_offset_and_depth(
    tokens: &mut Vec<PtxToken>,
    offset: Option<&Offset>,
    depth_compare: Option<&RegisterOperand>,
) {
    if let Some(offset) = offset {
        tokens.push(PtxToken::Comma);
        offset.unparse_tokens(tokens);
    }
    if let Some(depth_compare) = depth_compare {
        tokens.push(PtxToken::Comma);
        depth_compare.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Geometry {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Geometry::OneD => "1d",
            Geometry::TwoD => "2d",
            Geometry::ThreeD => "3d",
            Geometry::Array1D => "a1d",
            Geometry::Array2D => "a2d",
            Geometry::Cube => "cube",
            Geometry::ArrayCube => "acube",
            Geometry::TwoDMultisample => "2dms",
            Geometry::Array2DMultisample => "a2dms",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Vector4DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Vector4DataType::U32 => "u32",
            Vector4DataType::S32 => "s32",
            Vector4DataType::F16 => "f16",
            Vector4DataType::F32 => "f32",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for CoordinateType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            CoordinateType::S32 => "s32",
            CoordinateType::F32 => "f32",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Offset {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Offset::Scalar(operand) => operand.unparse_tokens(tokens),
            Offset::Pair(operands) => operands.unparse_tokens(tokens),
            Offset::Quad(operands) => operands.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for LevelOfDetail {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            LevelOfDetail::S32(operand) => operand.unparse_tokens(tokens),
            LevelOfDetail::F32(operand) => operand.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for GradientVector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            GradientVector::Scalar(operand) => operand.unparse_tokens(tokens),
            GradientVector::Pair(operands) => operands.unparse_tokens(tokens),
            GradientVector::Quad(operands) => operands.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Gradients {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.dpdx.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.dpdy.unparse_tokens(tokens);
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

impl PtxUnparser for Tex {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "tex");
        match self {
            Tex::Vector4ImplicitSampler(instr) => {
                push_vector4_header(
                    tokens,
                    None,
                    instr.geometry,
                    instr.data_type,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(tokens, &instr.texture, None, &instr.coordinates);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector4ExplicitSampler(instr) => {
                push_vector4_header(
                    tokens,
                    None,
                    instr.geometry,
                    instr.data_type,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    Some(&instr.sampler),
                    &instr.coordinates,
                );
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector2F16x2ImplicitSampler(instr) => {
                push_vector2_header(
                    tokens,
                    None,
                    instr.geometry,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(tokens, &instr.texture, None, &instr.coordinates);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector2F16x2ExplicitSampler(instr) => {
                push_vector2_header(
                    tokens,
                    None,
                    instr.geometry,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    Some(&instr.sampler),
                    &instr.coordinates,
                );
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector4MipBase(instr) => {
                push_vector4_header(
                    tokens,
                    Some("base"),
                    instr.geometry,
                    instr.data_type,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector4MipLevel(instr) => {
                push_vector4_header(
                    tokens,
                    Some("level"),
                    instr.geometry,
                    instr.data_type,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                tokens.push(PtxToken::Comma);
                instr.level_of_detail.unparse_tokens(tokens);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector4MipGradient(instr) => {
                push_vector4_header(
                    tokens,
                    Some("grad"),
                    instr.geometry,
                    instr.data_type,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                tokens.push(PtxToken::Comma);
                instr.gradients.unparse_tokens(tokens);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector2F16x2MipBase(instr) => {
                push_vector2_header(
                    tokens,
                    Some("base"),
                    instr.geometry,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector2F16x2MipLevel(instr) => {
                push_vector2_header(
                    tokens,
                    Some("level"),
                    instr.geometry,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                tokens.push(PtxToken::Comma);
                instr.level_of_detail.unparse_tokens(tokens);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
            Tex::Vector2F16x2MipGradient(instr) => {
                push_vector2_header(
                    tokens,
                    Some("grad"),
                    instr.geometry,
                    instr.coordinate_type,
                    &instr.destination,
                    instr.predicate.as_ref(),
                );
                push_texture_operands(
                    tokens,
                    &instr.texture,
                    instr.sampler.as_ref(),
                    &instr.coordinates,
                );
                tokens.push(PtxToken::Comma);
                instr.gradients.unparse_tokens(tokens);
                push_optional_offset_and_depth(
                    tokens,
                    instr.offset.as_ref(),
                    instr.depth_compare.as_ref(),
                );
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
