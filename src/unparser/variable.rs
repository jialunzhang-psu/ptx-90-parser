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
    unparser::{push_decimal, push_directive, push_identifier, push_space},
};

impl PtxUnparser for InitializerValue {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            InitializerValue::NumericLiteral {
                value: immediate, ..
            } => immediate.unparse_tokens_mode(tokens, spaced),
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
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            GlobalInitializer::Scalar { value, .. } => value.unparse_tokens_mode(tokens, spaced),
            GlobalInitializer::Aggregate {
                values: elements, ..
            } => {
                tokens.push(PtxToken::LBrace);
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        tokens.push(PtxToken::Comma);
                        push_space(tokens, spaced);
                    }
                    element.unparse_tokens_mode(tokens, spaced);
                }
                tokens.push(PtxToken::RBrace);
            }
        }
    }
}

impl PtxUnparser for VariableModifier {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            VariableModifier::Vector { value: width, .. } => {
                push_directive(tokens, &format!("v{width}"));
            }
            VariableModifier::Alignment { value, .. } => {
                push_directive(tokens, "align");
                push_space(tokens, spaced);
                push_decimal(tokens, value.to_string());
            }
            VariableModifier::Ptr { .. } => push_directive(tokens, "ptr"),
        }
    }
}

fn unparse_array_dimensions(tokens: &mut Vec<PtxToken>, extents: &[Option<u64>], spaced: bool) {
    let _ = spaced;
    for extent in extents {
        tokens.push(PtxToken::LBracket);
        if let Some(value) = extent {
            push_decimal(tokens, value.to_string());
        }
        tokens.push(PtxToken::RBracket);
    }
}

fn unparse_initializer(
    tokens: &mut Vec<PtxToken>,
    initializer: &Option<GlobalInitializer>,
    spaced: bool,
) {
    if let Some(initializer) = initializer {
        tokens.push(PtxToken::Equals);
        push_space(tokens, spaced);
        initializer.unparse_tokens_mode(tokens, spaced);
    }
}

fn unparse_prefix(
    tokens: &mut Vec<PtxToken>,
    linkage_modifiers: &[VariableModifier],
    other_modifiers: &[VariableModifier],
    attributes: &[AttributeDirective],
    spaced: bool,
) {
    for attribute in attributes {
        attribute.unparse_tokens_mode(tokens, spaced);
        push_space(tokens, spaced);
    }
    for modifier in linkage_modifiers {
        modifier.unparse_tokens_mode(tokens, spaced);
        push_space(tokens, spaced);
    }
    for modifier in other_modifiers {
        modifier.unparse_tokens_mode(tokens, spaced);
        push_space(tokens, spaced);
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
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        let (linkage_modifiers, other_modifiers) = split_modifiers(&self.modifiers);
        unparse_prefix(
            tokens,
            &linkage_modifiers,
            &other_modifiers,
            &self.attributes,
            spaced,
        );
        // Note: We don't remove any trailing space here. The caller (e.g., ModuleVariableDirective)
        // adds a space before us, and unparse_prefix adds spaces after each modifier. Both are needed
        // for proper spacing in the output.
        self.ty.unparse_tokens_mode(tokens, spaced);
        push_space(tokens, spaced);
        push_identifier(tokens, &self.name.val);
        unparse_array_dimensions(tokens, &self.array_dims, spaced);
        unparse_initializer(tokens, &self.initializer, spaced);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ModuleVariableDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            ModuleVariableDirective::Tex { directive, .. } => {
                push_directive(tokens, "tex");
                push_space(tokens, spaced);
                directive.unparse_tokens_mode(tokens, spaced);
            }
            ModuleVariableDirective::Shared { directive, .. } => {
                push_directive(tokens, "shared");
                push_space(tokens, spaced);
                directive.unparse_tokens_mode(tokens, spaced);
            }
            ModuleVariableDirective::Global { directive, .. } => {
                push_directive(tokens, "global");
                push_space(tokens, spaced);
                directive.unparse_tokens_mode(tokens, spaced);
            }
            ModuleVariableDirective::Const { directive, .. } => {
                push_directive(tokens, "const");
                push_space(tokens, spaced);
                directive.unparse_tokens_mode(tokens, spaced);
            }
        }
    }
}
