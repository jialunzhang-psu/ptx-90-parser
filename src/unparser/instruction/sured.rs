use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::sured::*},
    unparser::*,
};

fn unparse_coordinate_components(tokens: &mut Vec<PtxToken>, components: &[&RegisterOperand]) {
    if components.len() == 1 {
        components[0].unparse_tokens(tokens);
    } else {
        tokens.push(PtxToken::LBrace);
        for (index, component) in components.iter().enumerate() {
            if index > 0 {
                tokens.push(PtxToken::Comma);
            }
            component.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for Operator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Operator::Add => "add",
            Operator::Min => "min",
            Operator::Max => "max",
            Operator::And => "and",
            Operator::Or => "or",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for ByteDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            ByteDataType::U32 => "u32",
            ByteDataType::U64 => "u64",
            ByteDataType::S32 => "s32",
            ByteDataType::B32 => "b32",
            ByteDataType::S64 => "s64",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for SampleDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            SampleDataType::B32 => "b32",
            SampleDataType::B64 => "b64",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Clamp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            Clamp::Trap => "trap",
            Clamp::Clamp => "clamp",
            Clamp::Zero => "zero",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Surface {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Surface::Reference(symbol) => symbol.unparse_tokens(tokens),
            Surface::Indirect(register) => register.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Coordinate2d {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, &[&self.x, &self.y]);
    }
}

impl PtxUnparser for Coordinate3d {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, &[&self.x, &self.y, &self.z, &self.w]);
    }
}

fn unparse_reduction<TDataType, TCoordinates>(
    tokens: &mut Vec<PtxToken>,
    geometry: &str,
    reduction: &Reduction<TDataType, TCoordinates>,
) where
    TDataType: PtxUnparser,
    TCoordinates: PtxUnparser,
{
    reduction.operator.unparse_tokens(tokens);
    push_directive(tokens, geometry);
    reduction.data_type.unparse_tokens(tokens);
    reduction.clamp.unparse_tokens(tokens);
    tokens.push(PtxToken::LBracket);
    reduction.surface.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    reduction.coordinates.unparse_tokens(tokens);
    tokens.push(PtxToken::RBracket);
    tokens.push(PtxToken::Comma);
    reduction.source.unparse_tokens(tokens);
    tokens.push(PtxToken::Semicolon);
}

impl PtxUnparser for Sured {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "sured");
        match self {
            Sured::Byte1d(reduction) => {
                push_directive(tokens, "b");
                unparse_reduction(tokens, "1d", reduction);
            }
            Sured::Byte2d(reduction) => {
                push_directive(tokens, "b");
                unparse_reduction(tokens, "2d", reduction);
            }
            Sured::Byte3d(reduction) => {
                push_directive(tokens, "b");
                unparse_reduction(tokens, "3d", reduction);
            }
            Sured::Sample1d(reduction) => {
                push_directive(tokens, "p");
                unparse_reduction(tokens, "1d", reduction);
            }
            Sured::Sample2d(reduction) => {
                push_directive(tokens, "p");
                unparse_reduction(tokens, "2d", reduction);
            }
            Sured::Sample3d(reduction) => {
                push_directive(tokens, "p");
                unparse_reduction(tokens, "3d", reduction);
            }
        }
    }
}
