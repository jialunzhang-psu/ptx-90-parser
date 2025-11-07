use crate::analyzer::{
    AnalyzedInstruction, AnalyzedModifier, AnalyzedOperand, AnalyzedOperandElement, AnalyzedSection,
};
use crate::naming;
use crate::r#type::OperatorToken;

/// Output produced when generating unparsers for a PTX section
pub struct GeneratedUnparserOutput {
    /// Generated Rust code containing all unparser implementations for the section
    pub code: String,
    /// Struct names emitted for the instructions, used for dispatcher generation
    pub instruction_structs: Vec<String>,
    /// Module name for this section
    pub module_name: String,
}

/// Generator that emits `PtxUnparser` implementations for analyzed PTX instructions.
pub struct UnparserGenerator {
    /// Counter used to build unique binding names when destructuring tuples or enums.
    tmp_counter: usize,
}

impl UnparserGenerator {
    pub fn new() -> Self {
        Self { tmp_counter: 0 }
    }

    /// Generate unparser implementations for an analyzed section.
    pub fn generate(
        &mut self,
        section: &AnalyzedSection,
        section_idx: usize,
        type_module_name: &str,
    ) -> GeneratedUnparserOutput {
        let mut struct_names = Vec::new();
        let mut impl_output = String::new();

        for instr in &section.instructions {
            let struct_name = instr.rust_name.clone();
            struct_names.push(struct_name.clone());
            let impl_code = self.generate_instruction_unparser(instr);
            impl_output.push_str(&impl_code);
            impl_output.push_str("\n");
        }

        let module_name = format!("section_{}", section_idx);

        let mut output = String::new();
        output.push_str(&format!("pub mod {} {{\n", module_name));
        output.push_str("    use super::*;\n");
        output.push_str(&format!(
            "    use crate::r#type::instruction::{}::{}::*;\n\n",
            type_module_name, module_name
        ));

        for line in impl_output.lines() {
            if !line.is_empty() {
                output.push_str("    ");
            }
            output.push_str(line);
            output.push_str("\n");
        }

        output.push_str("}\n");

        GeneratedUnparserOutput {
            code: output,
            instruction_structs: struct_names,
            module_name,
        }
    }

    /// Generate helper imports used before section modules.
    pub fn generate_imports() -> String {
        "use crate::lexer::PtxToken;\nuse crate::unparser::{PtxUnparser, common::*};".to_string()
    }

    fn generate_instruction_unparser(&mut self, instr: &AnalyzedInstruction) -> String {
        let mut output = String::new();
        let struct_name = &instr.rust_name;
        let opcode = &instr.head.opcode;

        output.push_str(&format!("impl PtxUnparser for {} {{\n", struct_name));
        output.push_str("    fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {\n");
        output.push_str(&format!("        push_opcode(tokens, \"{}\");\n", opcode));

        for (modifier, rust_name) in &instr.head.modifiers {
            output.push_str(&self.generate_modifier_field_unparse(modifier, rust_name, 2));
        }

        // Operands are separated by commas – keep track if a comma is required before an operand.
        for (idx, operand) in instr.operands.iter().enumerate() {
            output.push_str(&self.generate_operand_unparse(operand, idx));
        }

        output.push_str("        tokens.push(PtxToken::Semicolon);\n");
        output.push_str("    }\n");
        output.push_str("}\n");

        output
    }

    fn generate_modifier_field_unparse(
        &mut self,
        modifier: &AnalyzedModifier,
        field_name: &str,
        indent: usize,
    ) -> String {
        match modifier {
            AnalyzedModifier::Optional(inner) => {
                let (inner_mod, _inner_name) = inner.as_ref();
                if matches!(inner_mod, AnalyzedModifier::Atom(_)) {
                    let mut code = String::new();
                    code.push_str(&format!(
                        "{}if self.{} {{\n",
                        self.indent(indent),
                        field_name
                    ));
                    code.push_str(&self.generate_modifier_value_unparse(inner_mod, "", indent + 1));
                    code.push_str(&format!("{}}}\n", self.indent(indent)));
                    code
                } else {
                    let binding = self.unique_binding(field_name);
                    let mut code = String::new();
                    code.push_str(&format!(
                        "{}if let Some({}) = self.{}.as_ref() {{\n",
                        self.indent(indent),
                        binding,
                        field_name
                    ));
                    code.push_str(&self.generate_modifier_value_unparse(
                        inner_mod,
                        &binding,
                        indent + 1,
                    ));
                    code.push_str(&format!("{}}}\n", self.indent(indent)));
                    code
                }
            }
            _ => self.generate_modifier_value_unparse(
                modifier,
                &format!("&self.{}", field_name),
                indent,
            ),
        }
    }

    fn generate_modifier_value_unparse(
        &mut self,
        modifier: &AnalyzedModifier,
        value_expr: &str,
        indent: usize,
    ) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => {
                format!(
                    "{}push_directive(tokens, \"{}\");\n",
                    self.indent(indent),
                    Self::directive_name(ident)
                )
            }
            AnalyzedModifier::Sequence(items) => self.emit_modifier_literal(items, indent),
            AnalyzedModifier::Choice { base, options } => {
                let enum_name = &base.1;
                let match_expr = if value_expr.is_empty() {
                    format!("&self.{}", naming::sanitize_field_name(&base.0))
                } else {
                    value_expr.to_string()
                };
                let mut code = String::new();
                code.push_str(&format!("{}match {} {{\n", self.indent(indent), match_expr));
                for (option, variant_name) in options {
                    code.push_str(&self.generate_choice_arm(
                        enum_name,
                        option,
                        variant_name,
                        indent + 1,
                    ));
                }
                code.push_str(&format!("{}}}\n", self.indent(indent)));
                code
            }
            AnalyzedModifier::Optional(inner) => {
                let (inner_mod, _) = inner.as_ref();
                self.generate_modifier_value_unparse(inner_mod, value_expr, indent)
            }
        }
    }

    fn generate_choice_arm(
        &mut self,
        enum_name: &str,
        option: &AnalyzedModifier,
        variant_name: &str,
        indent: usize,
    ) -> String {
        let indent_str = self.indent(indent);
        let mut code = String::new();
        let (pattern, bindings) = self.choice_variant_pattern_with_bindings(enum_name, variant_name, option);
        code.push_str(&format!("{}{} => {{\n", indent_str, pattern));
        code.push_str(&self.emit_literal_from_modifier_with_bindings(option, &bindings, indent + 1));
        code.push_str(&format!("{}}}\n", indent_str));
        code
    }

    fn generate_operand_unparse(&mut self, operand: &AnalyzedOperand, index: usize) -> String {
        let mut code = String::new();

        let operator_snippet = if let Some((operator, rust_name)) = &operand.operator {
            let token = match operator {
                OperatorToken::Negate => "Minus",
                OperatorToken::LogicalNot => "Exclaim",
            };
            Some(format!(
                "        if self.{} {{ tokens.push(PtxToken::{}); }}\n",
                rust_name, token
            ))
        } else {
            None
        };

        let field_name = Self::get_operand_rust_name(&operand.operand);
        let is_optional = Self::is_optional_operand(&operand.operand);

        if index > 0 {
            if is_optional {
                code.push_str(&format!(
                    "        if self.{field}.is_some() {{ tokens.push(PtxToken::Comma); }}\n",
                    field = field_name
                ));
            } else {
                code.push_str("        tokens.push(PtxToken::Comma);\n");
            }
        }

        if let Some(snippet) = operator_snippet {
            code.push_str(&snippet);
        }

        code.push_str(&self.generate_operand_element_unparse(
            &operand.operand,
            &format!("self.{}", field_name),
            2,
        ));

        if let Some((modifier, rust_name)) = &operand.modifier {
            code.push_str(&self.generate_modifier_field_unparse(modifier, rust_name, 2));
        }

        code
    }

    fn generate_operand_element_unparse(
        &mut self,
        element: &AnalyzedOperandElement,
        value_expr: &str,
        indent: usize,
    ) -> String {
        let mut code = String::new();
        let indent_str = self.indent(indent);

        match element {
            AnalyzedOperandElement::Item(_) | AnalyzedOperandElement::Address(_) => {
                code.push_str(&format!(
                    "{}{}.unparse_tokens(tokens);\n",
                    indent_str, value_expr
                ));
            }
            AnalyzedOperandElement::Optional(_) => {
                let binding = self.unique_binding("opt");
                code.push_str(&format!(
                    "{}if let Some({}) = {}.as_ref() {{\n",
                    indent_str, binding, value_expr
                ));
                code.push_str(&format!(
                    "{}    {}.unparse_tokens(tokens);\n",
                    indent_str, binding
                ));
                code.push_str(&format!("{}}}\n", indent_str));
            }
            AnalyzedOperandElement::ParenthesizedOperand(_) => {
                code.push_str(&format!("{}tokens.push(PtxToken::LParen);\n", indent_str));
                code.push_str(&format!(
                    "{}{}.unparse_tokens(tokens);\n",
                    indent_str, value_expr
                ));
                code.push_str(&format!("{}tokens.push(PtxToken::RParen);\n", indent_str));
            }
            AnalyzedOperandElement::PipeChoice(((_, first_name), (_, second_name))) => {
                // d|p outputs: d.unparse_tokens(), Pipe, p.unparse_tokens()
                code.push_str(&format!(
                    "{}self.{}.unparse_tokens(tokens);\n",
                    indent_str, first_name
                ));
                code.push_str(&format!("{}tokens.push(PtxToken::Pipe);\n", indent_str));
                code.push_str(&format!(
                    "{}self.{}.unparse_tokens(tokens);\n",
                    indent_str, second_name
                ));
            }
            AnalyzedOperandElement::PipeOptionalChoice(((_, _first_name), (_, second_name))) => {
                // Base operand
                code.push_str(&format!(
                    "{}{}.unparse_tokens(tokens);\n",
                    indent_str, value_expr
                ));

                // Optional guard
                let binding = self.unique_binding(second_name);
                code.push_str(&format!(
                    "{}if let Some({}) = self.{}.as_ref() {{\n",
                    indent_str, binding, second_name
                ));
                code.push_str(&format!("{}    tokens.push(PtxToken::Pipe);\n", indent_str));
                code.push_str(&format!(
                    "{}    {}.unparse_tokens(tokens);\n",
                    indent_str, binding
                ));
                code.push_str(&format!("{}}}\n", indent_str));
            }
            AnalyzedOperandElement::SquareGroup(_) | AnalyzedOperandElement::CurlyGroup(_) => {
                code.push_str(&format!(
                    "{}{}.unparse_tokens(tokens);\n",
                    indent_str, value_expr
                ));
            }
            AnalyzedOperandElement::ParamList(_) => {
                let binding = self.unique_binding("params");
                code.push_str(&format!("{}tokens.push(PtxToken::LParen);\n", indent_str));
                code.push_str(&format!(
                    "{}for (idx, operand) in {}.iter().enumerate() {{\n",
                    indent_str, value_expr
                ));
                code.push_str(&format!(
                    "{}    if idx > 0 {{ tokens.push(PtxToken::Comma); }}\n",
                    indent_str
                ));
                code.push_str(&format!(
                    "{}    operand.unparse_tokens(tokens);\n",
                    indent_str
                ));
                code.push_str(&format!("{}}}\n", indent_str));
                code.push_str(&format!("{}tokens.push(PtxToken::RParen);\n", indent_str));
                let _ = binding;
            }
            AnalyzedOperandElement::ImmediateNumber((num, _)) => {
                code.push_str(&format!(
                    "{}push_token_from_str(tokens, \"{}\");\n",
                    indent_str, num
                ));
            }
            AnalyzedOperandElement::Choice { base, options } => {
                let enum_name = &base.1;
                code.push_str(&format!("{}match &{} {{\n", indent_str, value_expr));
                for (option_ident, variant_name) in options {
                    code.push_str(&format!(
                        "{}    {}::{} => {{\n",
                        indent_str, enum_name, variant_name
                    ));
                    code.push_str(&self.emit_operand_choice_literal(option_ident, indent + 2));
                    code.push_str(&format!("{}    }}\n", indent_str));
                }
                code.push_str(&format!("{}}}\n", indent_str));
            }
        }
        code
    }

    fn indent(&self, level: usize) -> String {
        "        ".repeat(level)
    }

    fn unique_binding(&mut self, seed: &str) -> String {
        let name = naming::sanitize_field_name(seed);
        let result = if name.is_empty() {
            format!("tmp{}", self.tmp_counter)
        } else {
            format!("{}_{}", name, self.tmp_counter)
        };
        self.tmp_counter += 1;
        result
    }

    fn directive_name(raw: &str) -> String {
        raw.trim_start_matches('.').to_string()
    }

    fn get_operand_rust_name(operand: &AnalyzedOperandElement) -> String {
        match operand {
            AnalyzedOperandElement::Item((_, rust_name))
            | AnalyzedOperandElement::Address((_, rust_name))
            | AnalyzedOperandElement::Optional((_, rust_name))
            | AnalyzedOperandElement::ParenthesizedOperand((_, rust_name)) => rust_name.clone(),
            AnalyzedOperandElement::PipeChoice(((_, first_name), _))
            | AnalyzedOperandElement::PipeOptionalChoice(((_, first_name), _)) => {
                first_name.clone()
            }
            AnalyzedOperandElement::CurlyGroup(items) => items
                .first()
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| "group".to_string()),
            AnalyzedOperandElement::SquareGroup(items) => items
                .first()
                .map(|(_, name, _)| name.clone())
                .unwrap_or_else(|| "group".to_string()),
            AnalyzedOperandElement::ImmediateNumber((_, rust_name)) => rust_name.clone(),
            AnalyzedOperandElement::ParamList(rust_name) => rust_name.clone(),
            AnalyzedOperandElement::Choice { base, .. } => naming::to_snake_case(&base.1),
        }
    }

    fn is_optional_operand(element: &AnalyzedOperandElement) -> bool {
        matches!(element, AnalyzedOperandElement::Optional(_))
    }

    fn emit_modifier_literal(
        &mut self,
        items: &[(AnalyzedModifier, String)],
        indent: usize,
    ) -> String {
        let mut code = String::new();
        for (item, _) in items {
            code.push_str(&self.emit_literal_from_modifier(item, indent));
        }
        code
    }

    fn emit_literal_from_modifier(&mut self, modifier: &AnalyzedModifier, indent: usize) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => format!(
                "{}{}\n",
                self.indent(indent),
                self.format_literal_push(ident)
            ),
            AnalyzedModifier::Sequence(seq) => self.emit_modifier_literal(seq, indent),
            AnalyzedModifier::Optional(inner) => {
                let (nested, _) = inner.as_ref();
                self.emit_literal_from_modifier(nested, indent)
            }
            AnalyzedModifier::Choice { options, .. } => {
                let mut code = String::new();
                for (option, _) in options {
                    code.push_str(&self.emit_literal_from_modifier(option, indent));
                }
                code
            }
        }
    }

    fn format_literal_push(&self, ident: &str) -> String {
        if ident.starts_with('.') {
            format!(
                "push_directive(tokens, \"{}\");",
                Self::directive_name(ident)
            )
        } else {
            format!("push_token_from_str(tokens, \"{}\");", ident)
        }
    }

    fn emit_operand_choice_literal(&self, option: &str, indent: usize) -> String {
        let indent_str = self.indent(indent);
        if option.starts_with('.') {
            format!(
                "{}push_directive(tokens, \"{}\");\n",
                indent_str,
                option.trim_start_matches('.')
            )
        } else {
            format!(
                "{}push_token_from_str(tokens, \"{}\");\n",
                indent_str, option
            )
        }
    }

    fn choice_variant_pattern(
        &self,
        enum_name: &str,
        variant_name: &str,
        option: &AnalyzedModifier,
    ) -> String {
        self.choice_variant_pattern_with_bindings(enum_name, variant_name, option).0
    }

    fn choice_variant_pattern_with_bindings(
        &self,
        enum_name: &str,
        variant_name: &str,
        option: &AnalyzedModifier,
    ) -> (String, Vec<String>) {
        match option {
            AnalyzedModifier::Sequence(items) => {
                if items.is_empty() {
                    (format!("{}::{}()", enum_name, variant_name), vec![])
                } else {
                    // Check if this is a .b.n.n.n.n pattern that needs special handling
                    let needs_bindings = self.is_combinable_sequence(items);
                    
                    if needs_bindings {
                        // Bind the actual values so we can combine them
                        let bindings: Vec<String> = (0..items.len())
                            .map(|i| if i == 0 { "_".to_string() } else { format!("n{}", i) })
                            .collect();
                        let args = bindings.join(", ");
                        (format!("{}::{}({})", enum_name, variant_name, args), bindings)
                    } else {
                        // No bindings needed for regular sequences
                        let args = (0..items.len()).map(|_| "_").collect::<Vec<_>>().join(", ");
                        (format!("{}::{}({})", enum_name, variant_name, args), vec![])
                    }
                }
            }
            AnalyzedModifier::Optional(inner) => {
                let (inner_pattern, bindings) = self.choice_variant_pattern_with_bindings(enum_name, variant_name, &inner.0);
                if inner_pattern.contains("(") {
                    (inner_pattern, bindings)
                } else {
                    (format!("{}::{}(..)", enum_name, variant_name), vec![])
                }
            }
            AnalyzedModifier::Choice { .. } => (format!("{}::{}(..)", enum_name, variant_name), vec![]),
            _ => (format!("{}::{}", enum_name, variant_name), vec![]),
        }
    }

    /// Check if a sequence needs to be combined into a single identifier
    /// Returns true for patterns like .b.n.n.n.n where we have a directive followed by placeholders
    fn is_combinable_sequence(&self, items: &[(AnalyzedModifier, String)]) -> bool {
        if items.len() < 2 {
            return false;
        }
        
        // First item should be a directive starting with .
        let first_is_directive = matches!(&items[0].0, AnalyzedModifier::Atom((ident, _)) if ident.starts_with('.'));
        
        // Rest should be Choice items (like .n = { 0, 1, 2, ... })
        let rest_are_choices = items[1..].iter().all(|(modifier, _)| {
            matches!(modifier, AnalyzedModifier::Choice { .. })
        });
        
        first_is_directive && rest_are_choices
    }

    /// Generate code to convert an enum value to a character and append to `combined`
    /// The enum type should have variants that end with the character they represent
    /// (e.g., _0, _1, etc. or Buffer, Op, etc.)
    fn generate_enum_to_char(&self, binding: &str, indent: usize) -> String {
        let indent_str = self.indent(indent);
        // Extract the last character/digit from the debug representation of the enum variant
        // For _0 -> "0", for _7 -> "7", for Buffer -> "r", etc.
        format!("{}combined.push_str(format!(\"{{:?}}\", {}).trim_start_matches('_'));\n", indent_str, binding)
    }

    fn emit_literal_from_modifier_with_bindings(
        &mut self,
        modifier: &AnalyzedModifier,
        bindings: &[String],
        indent: usize,
    ) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => format!(
                "{}{}\n",
                self.indent(indent),
                self.format_literal_push(ident)
            ),
            AnalyzedModifier::Sequence(seq) => {
                // Check if this is a combinable sequence (like .b.n.n.n.n)
                if self.is_combinable_sequence(seq) && !bindings.is_empty() {
                    // Generate code to combine into a single identifier
                    let directive_name = if let AnalyzedModifier::Atom((ident, _)) = &seq[0].0 {
                        Self::directive_name(ident)
                    } else {
                        "b".to_string()  // fallback
                    };
                    
                    let mut code = String::new();
                    let indent_str = self.indent(indent);
                    
                    // Build the combined string by converting each enum value to its string representation
                    // We'll build it piece by piece
                    code.push_str(&format!("{}let mut combined = String::new();\n", indent_str));
                    
                    for binding in &bindings[1..] {  // Skip first binding which is "_"
                        // Each binding is an enum, we need to match it and convert to string
                        code.push_str(&self.generate_enum_to_char(binding, indent));
                    }
                    
                    // Push as DOT + identifier (two tokens)
                    code.push_str(&format!("{}tokens.push(PtxToken::Dot);\n", indent_str));
                    code.push_str(&format!("{}tokens.push(PtxToken::Identifier(format!(\"{{}}{{}}\", \"{}\", combined).into()));\n", indent_str, directive_name));
                    code
                } else {
                    self.emit_modifier_literal(seq, indent)
                }
            },
            AnalyzedModifier::Optional(inner) => {
                let (nested, _) = inner.as_ref();
                self.emit_literal_from_modifier_with_bindings(nested, bindings, indent)
            }
            AnalyzedModifier::Choice { options, .. } => {
                let mut code = String::new();
                for (option, _) in options {
                    code.push_str(&self.emit_literal_from_modifier_with_bindings(option, bindings, indent));
                }
                code
            }
        }
    }
}

/// Generate the content for `src/unparser/instruction/mod.rs`.
pub fn generate_unparser_mod_rs_content(modules: &[(String, Vec<(String, String)>)]) -> String {
    let mut output = String::new();
    output.push_str("// Auto-generated module declarations\n");
    output.push_str("// DO NOT EDIT MANUALLY\n");
    output.push_str("#![allow(unused)]\n\n");
    output.push_str("use crate::lexer::PtxToken;\n");
    output.push_str("use crate::unparser::PtxUnparser;\n");
    output.push_str("use crate::r#type::instruction::Inst;\n\n");

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {};\n", module_name));
    }
    output.push_str("\n");

    output.push_str("impl PtxUnparser for Inst {\n");
    output.push_str("    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {\n");
    output.push_str("        match self {\n");

    for (_module_name, structs) in modules {
        for (_section_name, struct_name) in structs {
            output.push_str(&format!(
                "            Inst::{}(value) => value.unparse_tokens(tokens),\n",
                struct_name
            ));
        }
    }

    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    output
}
