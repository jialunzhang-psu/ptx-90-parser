use crate::{
    lexer::PtxToken,
    r#type::{common::RegisterOperand, instruction::suld::*},
    unparser::*,
};

fn push_geometry_descriptor<TCoordinates>(
    tokens: &mut Vec<PtxToken>,
    geometry: &str,
    descriptor: &Descriptor<TCoordinates>,
) where
    TCoordinates: PtxUnparser,
{
    push_directive(tokens, geometry);
    descriptor.unparse_tokens(tokens);
}

fn unparse_coordinate_components<const N: usize>(
    tokens: &mut Vec<PtxToken>,
    components: [&RegisterOperand; N],
) {
    if N == 1 {
        components[0].unparse_tokens(tokens);
    } else {
        tokens.push(PtxToken::LBrace);
        for (index, component) in components.into_iter().enumerate() {
            if index > 0 {
                tokens.push(PtxToken::Comma);
            }
            component.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for CacheOperator {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            CacheOperator::Ca => "ca",
            CacheOperator::Cg => "cg",
            CacheOperator::Cs => "cs",
            CacheOperator::Cv => "cv",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for Vector {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Vector::None => {}
            Vector::V2 => push_directive(tokens, "v2"),
            Vector::V4 => push_directive(tokens, "v4"),
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            DataType::B8 => "b8",
            DataType::B16 => "b16",
            DataType::B32 => "b32",
            DataType::B64 => "b64",
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
            Surface::Register(register) => register.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Coordinate1d {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, [&self.x]);
    }
}

impl PtxUnparser for Coordinate2d {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, [&self.x, &self.y]);
    }
}

impl PtxUnparser for Coordinate3d {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, [&self.x, &self.y, &self.z, &self.w]);
    }
}

impl PtxUnparser for Array1dCoordinates {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, [&self.index, &self.x]);
    }
}

impl PtxUnparser for Array2dCoordinates {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_coordinate_components(tokens, [&self.index, &self.x, &self.y, &self.z]);
    }
}

impl<TCoordinates> PtxUnparser for Descriptor<TCoordinates>
where
    TCoordinates: PtxUnparser,
{
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Some(cache) = self.cache_operator {
            cache.unparse_tokens(tokens);
        }
        self.vector.unparse_tokens(tokens);
        self.data_type.unparse_tokens(tokens);
        self.clamp.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        tokens.push(PtxToken::LBracket);
        self.surface.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.coordinates.unparse_tokens(tokens);
        tokens.push(PtxToken::RBracket);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Suld {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "suld");
        push_directive(tokens, "b");
        match self {
            Suld::OneD(descriptor) => push_geometry_descriptor(tokens, "1d", descriptor),
            Suld::TwoD(descriptor) => push_geometry_descriptor(tokens, "2d", descriptor),
            Suld::ThreeD(descriptor) => push_geometry_descriptor(tokens, "3d", descriptor),
            Suld::Array1D(descriptor) => push_geometry_descriptor(tokens, "a1d", descriptor),
            Suld::Array2D(descriptor) => push_geometry_descriptor(tokens, "a2d", descriptor),
        }
    }
}
