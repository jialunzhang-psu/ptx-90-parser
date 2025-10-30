use super::PtxUnparser;
use crate::{
    lexer::PtxToken,
    r#type::{
        common::{AddressSpace, AttributeDirective},
        variable::{
            GlobalInitializer, InitializerValue, ModuleVariableDirective, NumericLiteral,
            VariableDirective, VariableModifier,
        },
    },
    unparser::{push_decimal, push_directive, push_identifier},
};

fn push_numeric_literal(tokens: &mut Vec<PtxToken>, literal: &NumericLiteral) {
    match literal {
        NumericLiteral::Signed(value) => {
            if *value < 0 {
                tokens.push(PtxToken::Minus);
                let magnitude = (*value as i128).abs();
                push_decimal(tokens, magnitude.to_string());
            } else {
                push_decimal(tokens, value.to_string());
            }
        }
        NumericLiteral::Unsigned(value) => {
            push_decimal(tokens, value.to_string());
        }
        NumericLiteral::Float64(bits) => {
            let text = format!("0d{:016x}", bits);
            tokens.push(PtxToken::HexFloat(text));
        }
        NumericLiteral::Float32(bits) => {
            let text = format!("0f{:08x}", bits);
            tokens.push(PtxToken::HexFloat(text));
        }
    }
}

impl PtxUnparser for NumericLiteral {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_numeric_literal(tokens, self);
    }
}

impl PtxUnparser for InitializerValue {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            InitializerValue::Numeric(literal) => literal.unparse_tokens(tokens),
            InitializerValue::Symbol(symbol) => push_identifier(tokens, symbol),
            InitializerValue::StringLiteral(value) => {
                tokens.push(PtxToken::StringLiteral(value.clone()));
            }
        }
    }
}

impl PtxUnparser for GlobalInitializer {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            GlobalInitializer::Scalar(value) => value.unparse_tokens(tokens),
            GlobalInitializer::Aggregate(elements) => {
                tokens.push(PtxToken::LBrace);
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    element.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
        }
    }
}

impl PtxUnparser for VariableModifier {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            VariableModifier::Vector(width) => {
                push_directive(tokens, &format!("v{width}"));
            }
            VariableModifier::Alignment(value) => {
                push_directive(tokens, "align");
                push_decimal(tokens, value.to_string());
            }
            VariableModifier::Linkage(linkage) => linkage.unparse_tokens(tokens),
            VariableModifier::Ptr => push_directive(tokens, "ptr"),
        }
    }
}

fn unparse_array_dimensions(tokens: &mut Vec<PtxToken>, extents: &[Option<u64>]) {
    for extent in extents {
        tokens.push(PtxToken::LBracket);
        if let Some(value) = extent {
            push_decimal(tokens, value.to_string());
        }
        tokens.push(PtxToken::RBracket);
    }
}

fn unparse_initializer(tokens: &mut Vec<PtxToken>, initializer: &Option<GlobalInitializer>) {
    if let Some(initializer) = initializer {
        tokens.push(PtxToken::Equals);
        initializer.unparse_tokens(tokens);
    }
}

fn unparse_prefix(
    tokens: &mut Vec<PtxToken>,
    linkage_modifiers: &[VariableModifier],
    other_modifiers: &[VariableModifier],
    attributes: &[AttributeDirective],
    address_space: &Option<AddressSpace>,
) {
    for attribute in attributes {
        attribute.unparse_tokens(tokens);
    }
    for modifier in linkage_modifiers {
        modifier.unparse_tokens(tokens);
    }
    if let Some(space) = address_space {
        space.unparse_tokens(tokens);
    }
    for modifier in other_modifiers {
        modifier.unparse_tokens(tokens);
    }
}

fn split_modifiers(
    modifiers: &[VariableModifier],
) -> (Vec<VariableModifier>, Vec<VariableModifier>) {
    let mut linkage = Vec::new();
    let mut other = Vec::new();
    for modifier in modifiers {
        match modifier {
            VariableModifier::Linkage(_) => linkage.push(modifier.clone()),
            _ => other.push(modifier.clone()),
        }
    }
    (linkage, other)
}

impl PtxUnparser for VariableDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let (linkage_modifiers, other_modifiers) = split_modifiers(&self.modifiers);
        unparse_prefix(
            tokens,
            &linkage_modifiers,
            &other_modifiers,
            &self.attributes,
            &self.address_space,
        );

        if let Some(ty) = &self.ty {
            ty.unparse_tokens(tokens);
        }

        push_identifier(tokens, &self.name);
        unparse_array_dimensions(tokens, &self.array);
        unparse_initializer(tokens, &self.initializer);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ModuleVariableDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleVariableDirective::Tex(directive) => {
                push_directive(tokens, "tex");
                directive.unparse_tokens(tokens);
            }
            ModuleVariableDirective::Shared(directive)
            | ModuleVariableDirective::Global(directive)
            | ModuleVariableDirective::Const(directive) => directive.unparse_tokens(tokens),
        }
    }
}
