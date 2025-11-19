use crate::analyzer::{
    AnalyzedInstruction, AnalyzedModifier, AnalyzedOperand, AnalyzedOperandElement, AnalyzedSection,
};
use crate::naming;
use crate::r#type::OperatorToken;
use std::collections::BTreeMap;
use std::fmt::Write;

/// Output produced when generating parsers for a PTX section
pub struct GeneratedParserOutput {
    /// Generated Rust code containing all parser implementations for the section
    pub code: String,
    /// Struct names emitted for the instructions, used for dispatcher generation
    pub instruction_structs: Vec<String>,
    /// Module name for this section
    pub module_name: String,
}

/// Parser generator for PTX specifications
pub struct ParserGenerator {
    /// Track generated enum parsers to avoid duplicates within a section
    generated_enum_parsers: BTreeMap<String, String>,
}

#[derive(Clone)]
struct Component {
    expr: String,
    binding: String,
    field_name: Option<String>,
}

impl Component {
    fn field<T, U>(expr: T, binding: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        let binding = binding.into();
        Self {
            expr: expr.into(),
            field_name: Some(binding.clone()),
            binding,
        }
    }

    fn ignore<T>(expr: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            expr: expr.into(),
            field_name: None,
            binding: "_".to_string(),
        }
    }
}

impl ParserGenerator {
    pub fn new() -> Self {
        Self {
            generated_enum_parsers: BTreeMap::new(),
        }
    }

    /// Generate parser implementations for an analyzed section
    pub fn generate(
        &mut self,
        section: &AnalyzedSection,
        section_idx: usize,
        type_module_name: &str,
    ) -> GeneratedParserOutput {
        let mut struct_output = String::new();
        let mut struct_names = Vec::new();

        // Generate parser implementation for each instruction
        for instr in &section.instructions {
            let struct_name = instr.rust_name.clone();
            let parser_impl = self.generate_parser_impl(instr);
            struct_names.push(struct_name);
            struct_output.push_str(&parser_impl);
            struct_output.push_str("\n\n");
        }

        // Add enum parsers for this section
        let mut enum_parsers_output = String::new();
        if !self.generated_enum_parsers.is_empty() {
            enum_parsers_output.push_str(
                "// ============================================================================\n",
            );
            enum_parsers_output.push_str("// Generated enum parsers\n");
            enum_parsers_output.push_str("// ============================================================================\n\n");
            for (_name, parser) in &self.generated_enum_parsers {
                enum_parsers_output.push_str(&parser);
                enum_parsers_output.push_str("\n");
            }
        }

        // Clear generated parsers for the next section
        self.generated_enum_parsers.clear();

        // Generate module name from section index
        let module_name = format!("section_{}", section_idx);

        // Wrap everything in a module
        let mut output = String::new();
        output.push_str(&format!("pub mod {} {{\n", module_name));
        output.push_str("    use super::*;\n");
        output.push_str(&format!(
            "    use crate::r#type::instruction::{}::{}::*;\n\n",
            type_module_name, module_name
        ));

        // Indent enum parsers
        for line in enum_parsers_output.lines() {
            if !line.is_empty() {
                output.push_str("    ");
            }
            output.push_str(line);
            output.push_str("\n");
        }

        // Indent parser implementations
        for line in struct_output.lines() {
            if !line.is_empty() {
                output.push_str("    ");
            }
            output.push_str(line);
            output.push_str("\n");
        }

        output.push_str("}\n");

        GeneratedParserOutput {
            code: output,
            instruction_structs: struct_names,
            module_name,
        }
    }

    /// Generate import statements
    pub fn generate_imports() -> String {
        r#"use crate::{alt, ok, seq_n};
use crate::parser::{
    PtxParseError, PtxParser, PtxTokenStream, Span,
    util::{comma_p, directive_p, exclamation_p, lbracket_p, lparen_p, minus_p, optional, pipe_p, rbracket_p, rparen_p, semicolon_p, sep_by, string_p, between, map, try_map},
};
use crate::r#type::common::*;"#
            .to_string()
    }

    fn escape_literal(&self, value: &str) -> String {
        value.replace('\\', "\\\\").replace('\"', "\\\"")
    }

    fn string_parser(&self, value: &str) -> String {
        format!("string_p(\"{}\")", self.escape_literal(value))
    }

    fn map_to_bool(&self, expr: &str) -> String {
        format!(
            "map({}, |value, _| value.is_some())",
            expr
        )
    }

    fn optional_with_comma(&self, parser_expr: &str) -> String {
        format!(
            "map(optional(seq_n!(comma_p(), {})), |value, _| value.map(|(_, operand)| operand))",
            parser_expr
        )
    }

    fn build_seq_expr(&self, components: &[Component]) -> String {
        if components.len() == 1 {
            return components[0].expr.clone();
        }
        let joined = components
            .iter()
            .map(|component| format!("                {}", component.expr))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("seq_n!(\n{}\n            )", joined)
    }

    fn build_seq_from_exprs(&self, exprs: &[String]) -> String {
        if exprs.len() == 1 {
            return exprs[0].clone();
        }
        let joined = exprs
            .iter()
            .map(|expr| format!("                {}", expr))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("seq_n!(\n{}\n            )", joined)
    }

    fn is_combinable_sequence(items: &[(AnalyzedModifier, String)]) -> bool {
        if items.len() < 2 {
            return false;
        }
        let first_is_directive = matches!(&items[0].0, AnalyzedModifier::Atom((ident, _)) if ident.starts_with('.'));
        if let AnalyzedModifier::Atom((ident, _)) = &items[0].0 {
            if ident.starts_with(".b") {
                eprintln!("Sequence candidate items: {:?}", items);
            }
        }
        let rest_are_choices = items[1..].iter().all(|(modifier, _)| {
            matches!(modifier, AnalyzedModifier::Choice { .. })
        });
        first_is_directive && rest_are_choices
    }

    fn combinable_sequence_expr(
        &self,
        items: &[(AnalyzedModifier, String)],
    ) -> Option<String> {
        if !Self::is_combinable_sequence(items) {
            return None;
        }
        eprintln!("Generating combinable sequence parser for {:?}", items);

        let prefix = match &items[0].0 {
            AnalyzedModifier::Atom((ident, _)) => ident.clone(),
            _ => return None,
        };

        let mut parts = Vec::new();
        for (modifier, _) in &items[1..] {
            if let AnalyzedModifier::Choice { base, options } = modifier {
                let type_name = base.1.clone();
                let mut literals = Vec::new();
                let mut variants = Vec::new();

                for (option, variant_name) in options {
                    if let AnalyzedModifier::Atom((literal, _)) = option {
                        literals.push(literal.clone());
                        variants.push(variant_name.clone());
                    } else {
                        return None;
                    }
                }

                parts.push((type_name, literals, variants));
            } else {
                return None;
            }
        }

        let mut expr = String::new();
        expr.push_str("|stream| {\n");
        expr.push_str("            stream.try_with_span(|stream| {\n");
        expr.push_str("                stream.with_partial_token_mode(|stream| {\n");
        expr.push_str(&format!(
            "                    stream.expect_string(\"{}\")?;\n",
            self.escape_literal(&prefix)
        ));

        for (idx, (type_name, literals, variants)) in parts.iter().enumerate() {
            let literal_list = literals
                .iter()
                .map(|lit| format!("\"{}\"", self.escape_literal(lit)))
                .collect::<Vec<_>>()
                .join(", ");
            expr.push_str(&format!(
                "                    let part{idx} = match stream.expect_strings(&[{literal_list}])? {{\n"
            ));
            for (value_idx, variant_name) in variants.iter().enumerate() {
                expr.push_str(&format!(
                    "                        {} => {}::{},\n",
                    value_idx, type_name, variant_name
                ));
            }
            expr.push_str("                        _ => unreachable!(),\n");
            expr.push_str("                    };\n");
        }

        let tuple_parts = std::iter::once("()".to_string())
            .chain((0..parts.len()).map(|idx| format!("part{}", idx)))
            .collect::<Vec<_>>()
            .join(", ");
        expr.push_str(&format!("                    Ok(({}))\n", tuple_parts));
        expr.push_str("                })\n");
        expr.push_str("            })\n");
        expr.push_str("        }\n");

        Some(expr)
    }

    fn build_pattern(&self, components: &[Component]) -> String {
        let bindings = components
            .iter()
            .map(|c| c.binding.clone())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", bindings)
    }

    fn build_field_initializers(&self, components: &[Component]) -> String {
        let mut lines = String::new();
        for component in components {
            if let Some(field) = &component.field_name {
                writeln!(
                    &mut lines,
                    "                    {} = {},",
                    field, component.binding
                )
                .unwrap();
            }
        }
        lines
    }

    /// Generate PtxParser implementation for an instruction
    fn generate_parser_impl(&mut self, instr: &AnalyzedInstruction) -> String {
        let struct_name = &instr.rust_name;
        let mut components = Vec::new();

        components.push(Component::ignore(self.string_parser(&instr.head.opcode)));

        for (modifier, rust_name) in &instr.head.modifiers {
            let expr = self.modifier_field_expr(modifier);
            components.push(Component::field(expr, rust_name.clone()));
        }

        for (idx, operand) in instr.operands.iter().enumerate() {
            self.add_operand_components(idx, operand, &mut components);
        }

        components.push(Component::ignore("semicolon_p()".to_string()));

        let seq_expr = self.build_seq_expr(&components);
        let pattern = self.build_pattern(&components);
        let field_initializers = self.build_field_initializers(&components);

        format!(
            r#"impl PtxParser for {} {{
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {{
        try_map(
            {},
            |{}, span| {{
                ok!({} {{
{}
                }})
            }},
        )
    }}
}}
"#,
            struct_name, seq_expr, pattern, struct_name, field_initializers
        )
    }


    fn modifier_field_expr(&mut self, modifier: &AnalyzedModifier) -> String {
        match modifier {
            AnalyzedModifier::Optional(inner) => match &inner.0 {
                AnalyzedModifier::Atom((ident, _)) => {
                    let expr = format!("optional({})", self.string_parser(ident));
                    self.map_to_bool(&expr)
                }
                _ => {
                    let expr = self.modifier_value_expr(&inner.0);
                    format!("optional({})", expr)
                }
            },
            _ => self.modifier_value_expr(modifier),
        }
    }

    fn modifier_value_expr(&mut self, modifier: &AnalyzedModifier) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => self.string_parser(ident),
            AnalyzedModifier::Choice { base, options } => {
                let type_name = &base.1;
                if !self.generated_enum_parsers.contains_key(type_name) {
                    let enum_parser = self.generate_enum_parser(type_name, options);
                    self.generated_enum_parsers
                        .insert(type_name.clone(), enum_parser);
                }
                format!("{}::parse()", type_name)
            }
            AnalyzedModifier::Sequence(items) => {
                if let Some(expr) = self.combinable_sequence_expr(items) {
                    expr
                } else {
                    let exprs: Vec<String> = items
                        .iter()
                        .map(|(item, _)| self.modifier_value_expr(item))
                        .collect();
                    self.build_seq_from_exprs(&exprs)
                }
            }
            AnalyzedModifier::Optional(_) => unreachable!("optional handled separately"),
        }
    }

    fn add_operand_components(
        &mut self,
        idx: usize,
        operand: &AnalyzedOperand,
        components: &mut Vec<Component>,
    ) {
        if matches!(operand.operand, AnalyzedOperandElement::Optional(_)) {
            let binding = Self::get_operand_rust_name(&operand.operand);
            let expr = if idx == 0 {
                "map(optional(GeneralOperand::parse()), |value, _| value)".to_string()
            } else {
                self.optional_with_comma("GeneralOperand::parse()")
            };
            components.push(Component::field(expr, binding));
            return;
        }

        if idx > 0 {
            components.push(Component::ignore("comma_p()".to_string()));
        }

        if let Some((operator, rust_name)) = &operand.operator {
            let expr = self.operator_expr(operator);
            components.push(Component::field(expr, rust_name.clone()));
        }

        for component in self.operand_element_components(&operand.operand) {
            components.push(component);
        }

        if let Some((modifier, rust_name)) = &operand.modifier {
            let expr = self.modifier_field_expr(modifier);
            components.push(Component::field(expr, rust_name.clone()));
        }
    }

    fn operator_expr(&self, operator: &OperatorToken) -> String {
        let parser = match operator {
            OperatorToken::Negate => "minus_p",
            OperatorToken::LogicalNot => "exclamation_p",
        };
        let expr = format!("optional({}())", parser);
        self.map_to_bool(&expr)
    }

    fn operand_element_components(
        &mut self,
        element: &AnalyzedOperandElement,
    ) -> Vec<Component> {
        use AnalyzedOperandElement::*;
        match element {
            Item((_, rust_name)) => {
                vec![Component::field("GeneralOperand::parse()", rust_name.clone())]
            }
            Address((_, rust_name)) => {
                vec![Component::field("AddressOperand::parse()", rust_name.clone())]
            }
            Optional((_, rust_name)) => {
                vec![Component::field("optional(GeneralOperand::parse())", rust_name.clone())]
            }
            ParenthesizedOperand((_, rust_name)) => {
                vec![Component::field(
                    "between(lparen_p(), rparen_p(), GeneralOperand::parse())",
                    rust_name.clone(),
                )]
            }
            PipeChoice(((_, first_name), (_, second_name))) => {
                vec![
                    Component::field("GeneralOperand::parse()", first_name.clone()),
                    Component::ignore("pipe_p()"),
                    Component::field("GeneralOperand::parse()", second_name.clone()),
                ]
            }
            PipeOptionalChoice(((_, first_name), (_, second_name))) => {
                let expr = "map(optional(seq_n!(pipe_p(), GeneralOperand::parse())), |value, _| value.map(|(_, operand)| operand))".to_string();
                vec![
                    Component::field("GeneralOperand::parse()", first_name.clone()),
                    Component::field(expr, second_name.clone()),
                ]
            }
            SquareGroup(items) => {
                let binding = Self::get_operand_rust_name(element);
                vec![Component::field(self.square_group_expr(items), binding)]
            }
            CurlyGroup(_) => {
                let binding = Self::get_operand_rust_name(element);
                vec![Component::field("VectorOperand::parse()", binding)]
            }
            ParamList(rust_name) => {
                vec![Component::field(
                    "between(lparen_p(), rparen_p(), sep_by(GeneralOperand::parse(), comma_p()))",
                    rust_name.clone(),
                )]
            }
            ImmediateNumber((value, rust_name)) => {
                let expr = format!(
                    "map({}, |_, _| ())",
                    self.string_parser(value)
                );
                vec![Component::field(expr, rust_name.clone())]
            }
            Choice { base, options } => {
                let binding = Self::get_operand_rust_name(element);
                let type_name = &base.1;
                if !self.generated_enum_parsers.contains_key(type_name) {
                    let analyzed_options: Vec<(AnalyzedModifier, String)> = options
                        .iter()
                        .map(|(ident, variant)| {
                            (
                                AnalyzedModifier::Atom((ident.clone(), variant.clone())),
                                variant.clone(),
                            )
                        })
                        .collect();
                    let enum_parser = self.generate_enum_parser(type_name, &analyzed_options);
                    self.generated_enum_parsers
                        .insert(type_name.clone(), enum_parser);
                }
                vec![Component::field(format!("{}::parse()", type_name), binding)]
            }
        }
    }

    fn square_group_expr(&mut self, items: &[(String, String, bool)]) -> String {
        match items.len() {
            2 => "TexHandler2::parse()".to_string(),
            3 => {
                if items.iter().any(|(_, _, optional)| *optional) {
                    "TexHandler3Optional::parse()".to_string()
                } else {
                    "TexHandler3::parse()".to_string()
                }
            }
            _ => {
                let mut inner = Vec::new();
                for (idx, (_, rust_name, is_optional)) in items.iter().enumerate() {
                    if idx == 0 {
                        if *is_optional {
                            inner.push(Component::field(
                                "optional(GeneralOperand::parse())",
                                rust_name.clone(),
                            ));
                        } else {
                            inner.push(Component::field(
                                "GeneralOperand::parse()",
                                rust_name.clone(),
                            ));
                        }
                    } else if *is_optional {
                        inner.push(Component::field(
                            self.optional_with_comma("GeneralOperand::parse()"),
                            rust_name.clone(),
                        ));
                    } else {
                        inner.push(Component::ignore("comma_p()"));
                        inner.push(Component::field(
                            "GeneralOperand::parse()",
                            rust_name.clone(),
                        ));
                    }
                }
                let inner_seq = self.build_seq_expr(&inner);
                let inner_pattern = self.build_pattern(&inner);
                let tuple_fields = inner
                    .iter()
                    .filter_map(|component| component.field_name.clone())
                    .collect::<Vec<_>>();
                let tuple_expr = format!("({})", tuple_fields.join(", "));
                format!(
                    "map(between(lbracket_p(), rbracket_p(), {}), |{}, _| {})",
                    inner_seq, inner_pattern, tuple_expr
                )
            }
        }
    }

    fn enum_variant_expr(
        &mut self,
        enum_name: &str,
        variant_name: &str,
        modifier: &AnalyzedModifier,
    ) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => format!(
                "map({}, |_, _span| {}::{})",
                self.string_parser(ident),
                enum_name,
                variant_name
            ),
            AnalyzedModifier::Sequence(items) => {
                let seq_expr = if let Some(expr) = self.combinable_sequence_expr(items) {
                    expr
                } else {
                    let exprs: Vec<String> = items
                        .iter()
                        .map(|(item, _)| self.modifier_value_expr(item))
                        .collect();
                    self.build_seq_from_exprs(&exprs)
                };
                let bindings: Vec<String> = items
                    .iter()
                    .enumerate()
                    .map(|(idx, (_, rust_name))| {
                        if rust_name.is_empty() {
                            format!("part{}", idx)
                        } else {
                            rust_name.clone()
                        }
                    })
                    .collect();
                let pattern = format!("({})", bindings.join(", "));
                let args = bindings.join(", ");
                let constructor = if args.is_empty() {
                    format!("{}::{}", enum_name, variant_name)
                } else {
                    format!("{}::{}({})", enum_name, variant_name, args)
                };
                format!(
                    "map({}, |{}, _span| {})",
                    seq_expr, pattern, constructor
                )
            }
            AnalyzedModifier::Choice { .. } => {
                let expr = self.modifier_value_expr(modifier);
                format!(
                    "map({}, |value, _span| {}::{}(value))",
                    expr, enum_name, variant_name
                )
            }
            AnalyzedModifier::Optional(inner) => {
                self.enum_variant_expr(enum_name, variant_name, &inner.0)
            }
        }
    }

    fn generate_enum_parser(
        &mut self,
        enum_name: &str,
        options: &[(AnalyzedModifier, String)],
    ) -> String {
        let variants = options
            .iter()
            .map(|(modifier, variant)| self.enum_variant_expr(enum_name, variant, modifier))
            .collect::<Vec<_>>();
        let joined = variants
            .iter()
            .map(|expr| format!("            {}", expr))
            .collect::<Vec<_>>()
            .join(",\n");
        format!(
            "impl PtxParser for {} {{\n    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {{\n        alt!(\n{}\n        )\n    }}\n}}\n",
            enum_name, joined
        )
    }

    fn get_operand_rust_name(operand: &AnalyzedOperandElement) -> String {
        use AnalyzedOperandElement::*;
        match operand {
            Item((_, rust_name))
            | Address((_, rust_name))
            | Optional((_, rust_name))
            | ParenthesizedOperand((_, rust_name))
            | ImmediateNumber((_, rust_name)) => rust_name.clone(),
            PipeChoice(((_, first_name), _))
            | PipeOptionalChoice(((_, first_name), _)) => first_name.clone(),
            CurlyGroup(items) => items
                .first()
                .map(|(_, rust_name)| rust_name.clone())
                .unwrap_or_else(|| "group".to_string()),
            SquareGroup(items) => items
                .first()
                .map(|(_, rust_name, _)| rust_name.clone())
                .unwrap_or_else(|| "group".to_string()),
            ParamList(rust_name) => rust_name.clone(),
            Choice { base, .. } => naming::to_snake_case(&base.1),
        }
    }
}

/// Generate the content for parser mod.rs file
/// Takes a list of (module_name, opcodes, structs) tuples
pub fn generate_parser_mod_rs_content(
    modules: &[(String, Vec<String>, Vec<(String, String)>)],
) -> String {
    let mut output = String::new();

    output.push_str("// Auto-generated module declarations\n");
    output.push_str("// DO NOT EDIT MANUALLY\n");
    output.push_str("#![allow(unused)]\n\n");

    output.push_str("use crate::parser::{PtxParser, PtxParseError, PtxTokenStream, Span};\n");
    output.push_str("use crate::r#type::instruction::Inst;\n");
    output.push_str("use crate::lexer::PtxToken;\n\n");

    for (module_name, _, _) in modules {
        output.push_str(&format!("pub mod {};\n", module_name));
    }
    output.push_str("\n");

    output.push_str(
        "pub(crate) fn parse_instruction_inner(stream: &mut PtxTokenStream) -> Result<Inst, PtxParseError> {\n",
    );
    output.push_str("    let start_pos = stream.position();\n");
    output.push_str("    \n");
    output.push_str("    // Peek at the opcode to determine which parser to try\n");
    output.push_str(
        "    let opcode = if let Ok((PtxToken::Identifier(name), _)) = stream.peek() {\n",
    );
    output.push_str("        name.as_str()\n");
    output.push_str("    } else {\n");
    output.push_str(
        "        let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span::new(0, 0));\n",
    );
    output.push_str(
        "        return Err(crate::unexpected_value!(span, &[\"instruction opcode\"], \"not an identifier\"));\n",
    );
    output.push_str("    };\n");
    output.push_str("    \n");
    output.push_str("    // Dispatch based on opcode\n");
    output.push_str("    match opcode {\n");

    let mut opcode_map: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    for (module_name, opcodes, structs) in modules {
        for opcode in opcodes {
            for (section_name, struct_name) in structs {
                opcode_map
                    .entry(opcode.clone())
                    .or_insert_with(Vec::new)
                    .push((
                        module_name.clone(),
                        section_name.clone(),
                        struct_name.clone(),
                    ));
            }
        }
    }

    for (opcode, parsers) in opcode_map {
        output.push_str(&format!("        \"{}\" => {{\n", opcode));
        for (module_name, section_name, struct_name) in parsers {
            output.push_str("            stream.set_position(start_pos);\n");
            output.push_str(&format!(
                "            match <crate::r#type::instruction::{}::{}::{} as PtxParser>::parse()(stream) {{\n",
                module_name, section_name, struct_name
            ));
            output.push_str(&format!(
                "                Ok((inst, _)) => return Ok(Inst::{}(inst)),\n",
                struct_name
            ));
            output.push_str("                Err(_) => {}\n");
            output.push_str("            }\n");
        }
        output.push_str("        }\n");
    }

    output.push_str("        _ => {}\n");
    output.push_str("    }\n");
    output.push_str("    \n");
    output.push_str("    // If no parser matched, return error\n");
    output.push_str(
        "    let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span::new(0, 0));\n",
    );
    output.push_str(
        "    Err(crate::unexpected_value!(span, &[\"valid PTX instruction\"], \"no matching instruction format\"))\n",
    );
    output.push_str("}\n");
    output.push_str("\n");

    output.push_str("impl PtxParser for Inst {\n");
    output.push_str(
        "    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, Span), PtxParseError> {\n",
    );
    output.push_str("        |stream| stream.try_with_span(|stream| parse_instruction_inner(stream))\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    output.push_str("\n");

    output
}

/// Generate complete parser file from PTX specification content
/// Returns (generated_code, ParserModuleInfo(module_name, opcodes, instruction_structs))
pub fn generate_parser_file(
    spec_content: &str,
    file_name: &str,
    module_name: &str,
) -> Result<(String, (String, Vec<String>, Vec<(String, String)>)), Box<dyn std::error::Error>> {
    use crate::analyzer::Analyzer;
    use std::collections::BTreeSet;

    let sections = crate::parse_spec_with_name(spec_content, file_name)?;

    if sections.is_empty() {
        return Err("No sections found in file".into());
    }

    // Analyze all sections at once
    let mut analyzer = Analyzer::new();
    let analyzed_sections = analyzer.analyze_sections(&sections);

    if analyzed_sections.is_empty() {
        return Err("No instructions found".into());
    }

    // Collect ALL unique opcodes from ALL instructions in the file
    let opcodes: Vec<String> = analyzed_sections
        .iter()
        .flat_map(|section| section.opcodes.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    if opcodes.is_empty() {
        return Err("No instructions found".into());
    }

    // Generate parser definitions section by section
    let mut all_outputs = Vec::new();
    let mut parser_gen = ParserGenerator::new();

    for (section_idx, section) in analyzed_sections.iter().enumerate() {
        let generated = parser_gen.generate(section, section_idx, module_name);

        if !generated.code.trim().is_empty() {
            all_outputs.push(generated);
        }
    }

    if all_outputs.is_empty() {
        return Err("No instructions found".into());
    }

    // Collect all instruction structs with their section info
    let all_instruction_structs: Vec<(String, String)> = analyzed_sections
        .iter()
        .flat_map(|section| {
            let section_name = format!(
                "section_{}",
                analyzed_sections
                    .iter()
                    .position(|s| std::ptr::eq(s, section))
                    .unwrap()
            );
            section
                .instructions
                .iter()
                .map(move |instr| (section_name.clone(), instr.rust_name.clone()))
        })
        .collect();

    let mut output = String::new();
    output.push_str("//! Original PTX specification:\n");
    output.push_str("//!\n");
    for line in spec_content.lines() {
        output.push_str("//! ");
        output.push_str(line);
        output.push_str("\n");
    }
    output.push_str("\n");
    output.push_str("#![allow(unused)]\n");
    output.push_str("\n");

    // Add imports once
    output.push_str(&ParserGenerator::generate_imports());
    output.push_str("\n\n");

    // Append generated code for each section
    for gen_output in all_outputs.iter() {
        output.push_str(&gen_output.code);
        output.push_str("\n");
    }

    Ok((
        output,
        (module_name.to_string(), opcodes, all_instruction_structs),
    ))
}
