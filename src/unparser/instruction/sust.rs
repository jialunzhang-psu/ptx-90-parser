use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::sust::*},
    unparser::*,
};

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
            CacheOperator::Wb => "wb",
            CacheOperator::Cg => "cg",
            CacheOperator::Cs => "cs",
            CacheOperator::Wt => "wt",
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

impl PtxUnparser for ComponentType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let modifier = match self {
            ComponentType::B8 => "b8",
            ComponentType::B16 => "b16",
            ComponentType::B32 => "b32",
            ComponentType::B64 => "b64",
        };
        push_directive(tokens, modifier);
    }
}

impl PtxUnparser for FormattedComponentType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FormattedComponentType::B32 => push_directive(tokens, "b32"),
        }
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

impl<TCoordinates> PtxUnparser for Block<TCoordinates>
where
    TCoordinates: PtxUnparser,
{
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Some(cache) = self.cache_operator {
            cache.unparse_tokens(tokens);
        }
        self.vector.unparse_tokens(tokens);
        self.component_type.unparse_tokens(tokens);
        self.clamp.unparse_tokens(tokens);
        tokens.push(PtxToken::LBracket);
        self.surface.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.coordinates.unparse_tokens(tokens);
        tokens.push(PtxToken::RBracket);
        tokens.push(PtxToken::Comma);
        self.value.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl<TCoordinates> PtxUnparser for Formatted<TCoordinates>
where
    TCoordinates: PtxUnparser,
{
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.vector.unparse_tokens(tokens);
        self.component_type.unparse_tokens(tokens);
        self.clamp.unparse_tokens(tokens);
        tokens.push(PtxToken::LBracket);
        self.surface.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.coordinates.unparse_tokens(tokens);
        tokens.push(PtxToken::RBracket);
        tokens.push(PtxToken::Comma);
        self.value.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Sust {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "sust");
        match self {
            Sust::Block1d(block) => {
                push_directive(tokens, "b");
                push_directive(tokens, "1d");
                block.unparse_tokens(tokens);
            }
            Sust::Block2d(block) => {
                push_directive(tokens, "b");
                push_directive(tokens, "2d");
                block.unparse_tokens(tokens);
            }
            Sust::Block3d(block) => {
                push_directive(tokens, "b");
                push_directive(tokens, "3d");
                block.unparse_tokens(tokens);
            }
            Sust::BlockArray1d(block) => {
                push_directive(tokens, "b");
                push_directive(tokens, "a1d");
                block.unparse_tokens(tokens);
            }
            Sust::BlockArray2d(block) => {
                push_directive(tokens, "b");
                push_directive(tokens, "a2d");
                block.unparse_tokens(tokens);
            }
            Sust::Formatted1d(formatted) => {
                push_directive(tokens, "p");
                push_directive(tokens, "1d");
                formatted.unparse_tokens(tokens);
            }
            Sust::Formatted2d(formatted) => {
                push_directive(tokens, "p");
                push_directive(tokens, "2d");
                formatted.unparse_tokens(tokens);
            }
            Sust::Formatted3d(formatted) => {
                push_directive(tokens, "p");
                push_directive(tokens, "3d");
                formatted.unparse_tokens(tokens);
            }
        }
    }
}
