use super::PtxUnparser;
use crate::{
    lexer::PtxToken,
    r#type::{
        common::AttributeDirective,
        variable::{
            GlobalInitializer, InitializerValue, ModuleVariableDirective, VariableDirective,
            VariableModifier,
        },
    },
    unparser::{push_decimal, push_directive, push_identifier},
};

impl PtxUnparser for InitializerValue {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            InitializerValue::NumericLiteral {
                value: immediate, ..
            } => immediate.unparse_tokens(tokens),
            InitializerValue::FunctionSymbol { name: symbol, .. } => {
                push_identifier(tokens, &symbol.val)
            }
            InitializerValue::StringLiteral { value, .. } => {
                tokens.push(PtxToken::StringLiteral(value.clone()));
            }
        }
    }
}

impl PtxUnparser for GlobalInitializer {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            GlobalInitializer::Scalar { value, .. } => value.unparse_tokens(tokens),
            GlobalInitializer::Aggregate {
                values: elements, ..
            } => {
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
            VariableModifier::Vector { value: width, .. } => {
                push_directive(tokens, &format!("v{width}"));
            }
            VariableModifier::Alignment { value, .. } => {
                push_directive(tokens, "align");
                push_decimal(tokens, value.to_string());
            }
            VariableModifier::Ptr { .. } => push_directive(tokens, "ptr"),
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
) {
    for attribute in attributes {
        attribute.unparse_tokens(tokens);
    }
    for modifier in linkage_modifiers {
        modifier.unparse_tokens(tokens);
    }
    for modifier in other_modifiers {
        modifier.unparse_tokens(tokens);
    }
}

fn split_modifiers(
    modifiers: &[VariableModifier],
) -> (Vec<VariableModifier>, Vec<VariableModifier>) {
    // Linkage is now handled at module level, not in modifiers
    // All modifiers go to "other" category
    (Vec::new(), modifiers.to_vec())
}

impl PtxUnparser for VariableDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let (linkage_modifiers, other_modifiers) = split_modifiers(&self.modifiers);
        unparse_prefix(
            tokens,
            &linkage_modifiers,
            &other_modifiers,
            &self.attributes,
        );

        self.ty.unparse_tokens(tokens);

        push_identifier(tokens, &self.name.val);
        unparse_array_dimensions(tokens, &self.array_dims);
        unparse_initializer(tokens, &self.initializer);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ModuleVariableDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleVariableDirective::Tex { directive, .. } => {
                push_directive(tokens, "tex");
                directive.unparse_tokens(tokens);
            }
            ModuleVariableDirective::Shared { directive, .. } => {
                push_directive(tokens, "shared");
                directive.unparse_tokens(tokens);
            }
            ModuleVariableDirective::Global { directive, .. } => {
                push_directive(tokens, "global");
                directive.unparse_tokens(tokens);
            }
            ModuleVariableDirective::Const { directive, .. } => {
                push_directive(tokens, "const");
                directive.unparse_tokens(tokens);
            }
        }
    }
}
