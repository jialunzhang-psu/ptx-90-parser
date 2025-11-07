use crate::analyzer::{
    AnalyzedInstruction, AnalyzedModifier, AnalyzedOperand, AnalyzedOperandElement, AnalyzedSection,
};
use crate::naming;
use crate::r#type::OperatorToken;
use std::collections::BTreeMap;

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
        output.push_str(&format!("    use crate::r#type::instruction::{}::{}::*;\n\n", type_module_name, module_name));
        
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
        r#"use crate::lexer::PtxToken;
use crate::parser::{PtxParseError, PtxParser, PtxTokenStream, Span};
use crate::r#type::common::*;"#
            .to_string()
    }

    /// Generate PtxParser implementation for an instruction
    fn generate_parser_impl(&mut self, instr: &AnalyzedInstruction) -> String {
        let struct_name = &instr.rust_name;
        let opcode = &instr.head.opcode;

        let mut output = String::new();
        output.push_str(&format!("impl PtxParser for {} {{\n", struct_name));
        output.push_str(&format!("    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {{\n"));

        // Parse opcode
        output.push_str(&format!("        stream.expect_string(\"{}\")?;\n", opcode));

        // Parse modifiers
        for (modifier, rust_name) in &instr.head.modifiers {
            let parse_code = self.generate_modifier_parser(modifier, rust_name);
            output.push_str(&parse_code);
            // Ensure complete token consumption after each modifier
            output.push_str("        stream.expect_complete()?;\n");
        }

        // Parse operands
        for (idx, operand) in instr.operands.iter().enumerate() {
            // Add comma before each operand except the first
            if idx > 0 {
                // If the operand is optional, make the comma optional too
                let is_optional = matches!(
                    operand.operand,
                    AnalyzedOperandElement::Optional(_) | AnalyzedOperandElement::PipeOptionalChoice(_)
                );

                if is_optional {
                    output.push_str("        let saved_pos = stream.position();\n");
                    output.push_str("        let has_comma = stream.expect(&PtxToken::Comma).is_ok();\n");
                    output.push_str("        if !has_comma {\n");
                    output.push_str("            stream.set_position(saved_pos);\n");
                    output.push_str("        }\n");
                } else {
                    output.push_str("        stream.expect(&PtxToken::Comma)?;\n");
                }
            }
            let parse_code = self.generate_operand_parser(operand);
            output.push_str(&parse_code);
        }

        // Ensure we've consumed complete tokens (not stopped mid-token)
        output.push_str("        stream.expect_complete()?;\n");
        
        // Expect semicolon at end of instruction
        output.push_str("        stream.expect(&PtxToken::Semicolon)?;\n");

        // Construct and return the struct
        output.push_str(&format!("        Ok({} {{\n", struct_name));

        // Add modifier fields
        for (_modifier, rust_name) in &instr.head.modifiers {
            output.push_str(&format!("            {},\n", rust_name));
        }

        // Add operand fields
        for operand in &instr.operands {
            // Operator field
            if let Some((_op, rust_name)) = &operand.operator {
                output.push_str(&format!("            {},\n", rust_name));
            }

            // Operand element field(s)
            // PipeChoice and PipeOptionalChoice need TWO fields
            use AnalyzedOperandElement::*;
            match &operand.operand {
                PipeChoice(((_, first_name), (_, second_name))) => {
                    // Add both fields for d|p
                    output.push_str(&format!("            {},\n", first_name));
                    output.push_str(&format!("            {},\n", second_name));
                }
                PipeOptionalChoice(((_, first_name), (_, second_name))) => {
                    // Add both fields for d{|p}
                    output.push_str(&format!("            {},\n", first_name));
                    output.push_str(&format!("            {},\n", second_name));
                }
                _ => {
                    // Single field for all other types
                    let operand_name = Self::get_operand_rust_name(&operand.operand);
                    output.push_str(&format!("            {},\n", operand_name));
                }
            }

            // Operand modifier field
            if let Some((_modifier, rust_name)) = &operand.modifier {
                output.push_str(&format!("            {},\n", rust_name));
            }
        }

        output.push_str("        })\n");
        output.push_str("    }\n");
        output.push_str("}\n");

        output
    }

    /// Generate parser code for a modifier
    fn generate_modifier_parser(&mut self, modifier: &AnalyzedModifier, rust_name: &str) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => {
                format!(
                    "        stream.expect_string(\"{}\")?;\n        let {} = ();\n",
                    ident, rust_name
                )
            }
            AnalyzedModifier::Choice { base, options } => {
                let type_name = &base.1;
                
                // Generate enum parser if not already generated
                if !self.generated_enum_parsers.contains_key(type_name) {
                    let enum_parser = self.generate_enum_parser(type_name, options);
                    self.generated_enum_parsers.insert(type_name.clone(), enum_parser);
                }

                format!(
                    "        let {} = {}::parse(stream)?;\n",
                    rust_name, type_name
                )
            }
            AnalyzedModifier::Sequence(items) => {
                let mut code = String::new();
                let mut tuple_vars = Vec::new();

                for (item, item_rust_name) in items {
                    let item_code = self.generate_modifier_parser(item, item_rust_name);
                    code.push_str(&item_code);
                    tuple_vars.push(item_rust_name.clone());
                }

                // Ensure complete token consumption after sequence
                code.push_str("        stream.expect_complete()?;\n");

                let tuple_value = format!("({})", tuple_vars.join(", "));
                code.push_str(&format!("        let {} = {};\n", rust_name, tuple_value));

                code
            }
            AnalyzedModifier::Optional(inner_boxed) => {
                let (inner, inner_rust_name) = inner_boxed.as_ref();
                match inner {
                    AnalyzedModifier::Atom((ident, _)) => {
                        // Optional atom - try to match the directive
                        format!(
                            "        let saved_pos = stream.position();\n        let {} = stream.expect_string(\"{}\").is_ok();\n        if !{} {{\n            stream.set_position(saved_pos);\n        }}\n",
                            rust_name, ident, rust_name
                        )
                    }
                    AnalyzedModifier::Choice { base, options } => {
                        // For Choice types, generate enum parser if not already generated
                        let inner_type = &base.1;
                        if !self.generated_enum_parsers.contains_key(inner_type) {
                            let enum_parser = self.generate_enum_parser(inner_type, options);
                            self.generated_enum_parsers.insert(inner_type.clone(), enum_parser);
                        }
                        
                        // Use try-parse pattern
                        format!(
                            "        let saved_pos = stream.position();\n        let {} = match {}::parse(stream) {{\n            Ok(val) => Some(val),\n            Err(_) => {{\n                stream.set_position(saved_pos);\n                None\n            }}\n        }};\n",
                            rust_name, inner_type
                        )
                    }
                    _ => {
                        // For other types, parse the inner element and wrap in Option
                        let mut code = String::new();
                        code.push_str("        let saved_pos = stream.position();\n");
                        code.push_str(&format!("        let {} = match (|| {{\n", rust_name));
                        code.push_str(&self.generate_modifier_parser(inner, inner_rust_name));
                        code.push_str(&format!("            Ok::<_, PtxParseError>({})\n", inner_rust_name));
                        code.push_str("        })() {\n");
                        code.push_str("            Ok(val) => Some(val),\n");
                        code.push_str("            Err(_) => {\n");
                        code.push_str("                stream.set_position(saved_pos);\n");
                        code.push_str("                None\n");
                        code.push_str("            }\n");
                        code.push_str("        };\n");
                        code
                    }
                }
            }
        }
    }

    /// Generate parser code for an operand
    fn generate_operand_parser(&mut self, operand: &AnalyzedOperand) -> String {
        let mut code = String::new();

        // Parse operator prefix if present
        if let Some((operator, rust_name)) = &operand.operator {
            let op_token = match operator {
                OperatorToken::Negate => "Minus",
                OperatorToken::LogicalNot => "Exclaim",
            };
            code.push_str(&format!(
                "        let {} = stream.consume_if(|t| matches!(t, PtxToken::{})).is_some();\n",
                rust_name, op_token
            ));
        }

        // Parse operand element
        let operand_code = self.generate_operand_element_parser(&operand.operand);
        code.push_str(&operand_code);

        // Parse operand modifier if present
        if let Some((modifier, rust_name)) = &operand.modifier {
            let modifier_code = self.generate_modifier_parser(modifier, rust_name);
            code.push_str(&modifier_code);
        }
        
        // Ensure complete token consumption after each operand
        code.push_str("        stream.expect_complete()?;\n");

        code
    }

    /// Generate parser code for an operand element
    fn generate_operand_element_parser(&mut self, element: &AnalyzedOperandElement) -> String {
        use AnalyzedOperandElement::*;
        
        let rust_name = Self::get_operand_rust_name(element);

        match element {
            Item(_) => {
                format!("        let {} = GeneralOperand::parse(stream)?;\n", rust_name)
            }
            Address(_) => {
                format!(
                    "        let {} = AddressOperand::parse(stream)?;\n",
                    rust_name
                )
            }
            Optional((_ident, _)) => {
                format!(
                    "        let saved_pos = stream.position();\n        let {} = match GeneralOperand::parse(stream) {{\n            Ok(val) => Some(val),\n            Err(_) => {{\n                stream.set_position(saved_pos);\n                None\n            }}\n        }};\n",
                    rust_name
                )
            }
            ParenthesizedOperand(_) => {
                format!(
                    "        stream.expect(&PtxToken::LParen)?;\n        let {} = GeneralOperand::parse(stream)?;\n        stream.expect(&PtxToken::RParen)?;\n",
                    rust_name
                )
            }
            PipeChoice(((_, first_name), (_, second_name))) => {
                format!(
                    "        let {} = GeneralOperand::parse(stream)?;\n        stream.expect(&PtxToken::Pipe)?;\n        let {} = GeneralOperand::parse(stream)?;\n",
                    first_name, second_name
                )
            }
            PipeOptionalChoice(((_, first_name), (_, second_name))) => {
                format!(
                    "        let {} = GeneralOperand::parse(stream)?;\n        let saved_pos = stream.position();\n        let {} = if stream.consume_if(|t| matches!(t, PtxToken::Pipe)).is_some() {{\n            Some(GeneralOperand::parse(stream)?)\n        }} else {{\n            stream.set_position(saved_pos);\n            None\n        }};\n",
                    first_name, second_name
                )
            }
            SquareGroup(operands) => {
                match operands.len() {
                    2 => format!("        let {} = TexHandler2::parse(stream)?;\n", rust_name),
                    3 => {
                        if operands.iter().any(|(_, _, optional)| *optional) {
                            format!("        let {} = TexHandler3Optional::parse(stream)?;\n", rust_name)
                        } else {
                            format!("        let {} = TexHandler3::parse(stream)?;\n", rust_name)
                        }
                    }
                    _ => {
                        let mut code = String::new();
                        let mut field_names = Vec::new();

                        code.push_str("        stream.expect(&PtxToken::LBracket)?;\n");

                        for (idx, (_, field_name, _)) in operands.iter().enumerate() {
                            if idx > 0 {
                                code.push_str("        stream.expect(&PtxToken::Comma)?;\n");
                            }
                            code.push_str(&format!("        let {} = GeneralOperand::parse(stream)?;\n", field_name));
                            field_names.push(field_name.clone());
                        }

                        code.push_str("        stream.expect(&PtxToken::RBracket)?;\n");
                        code.push_str(&format!(
                            "        let {} = ({});\n",
                            rust_name,
                            field_names.join(", ")
                        ));

                        code
                    }
                }
            }
            CurlyGroup(_) => {
                format!("        let {} = VectorOperand::parse(stream)?;\n", rust_name)
            }
            ParamList(_) => {
                format!(
                    "        stream.expect(&PtxToken::LParen)?;\n        let mut {} = Vec::new();\n        // Parse comma-separated operands\n        loop {{\n            // Try to parse an operand\n            let saved_pos = stream.position();\n            match GeneralOperand::parse(stream) {{\n                Ok(operand) => {{\n                    {}.push(operand);\n                    // Check for comma\n                    if stream.expect(&PtxToken::Comma).is_err() {{\n                        break;\n                    }}\n                }}\n                Err(_) => {{\n                    stream.set_position(saved_pos);\n                    break;\n                }}\n            }}\n        }}\n        stream.expect(&PtxToken::RParen)?;\n",
                    rust_name, rust_name
                )
            }
            ImmediateNumber((num, _)) => {
                format!(
                    "        stream.expect_string(\"{}\")?;\n        let {} = ();\n",
                    num, rust_name
                )
            }
            Choice { base, options } => {
                let type_name = &base.1;
                
                // Generate enum parser if not already generated
                if !self.generated_enum_parsers.contains_key(type_name) {
                    // Convert to AnalyzedModifier format, variant names are already computed in analyzer
                    let analyzed_options: Vec<(AnalyzedModifier, String)> = options
                        .iter()
                        .map(|(ident, variant_name)| {
                            (AnalyzedModifier::Atom((ident.clone(), String::new())), variant_name.clone())
                        })
                        .collect();
                    let enum_parser = self.generate_enum_parser(type_name, &analyzed_options);
                    self.generated_enum_parsers.insert(type_name.clone(), enum_parser);
                }

                format!(
                    "        let {} = {}::parse(stream)?;\n",
                    rust_name, type_name
                )
            }
        }
    }

    /// Get the rust name from an operand element
    fn get_operand_rust_name(operand: &AnalyzedOperandElement) -> String {
        use AnalyzedOperandElement::*;
        match operand {
            Item((_, rust_name))
            | Address((_, rust_name))
            | Optional((_, rust_name))
            | ParenthesizedOperand((_, rust_name))
            | ImmediateNumber((_, rust_name)) => rust_name.clone(),
            PipeChoice(((_, first_name), _)) | PipeOptionalChoice(((_, first_name), _)) => first_name.clone(),
            CurlyGroup(items) => items.first().map(|(_, rust_name)| rust_name.clone()).unwrap_or_else(|| "group".to_string()),
            SquareGroup(items) => items.first().map(|(_, rust_name, _)| rust_name.clone()).unwrap_or_else(|| "group".to_string()),
            ParamList(rust_name) => rust_name.clone(),
            Choice { base, .. } => {
                // base.1 contains the PascalCase type name (e.g., "CpSize")
                // Convert to snake_case for use as variable/field name (e.g., "cp_size")
                naming::to_snake_case(&base.1)
            }
        }
    }

    /// Generate parser for an enum type (for Choice modifiers)
    fn generate_enum_parser(&mut self, enum_name: &str, options: &[(AnalyzedModifier, String)]) -> String {
        let mut output = String::new();

        output.push_str(&format!("impl PtxParser for {} {{\n", enum_name));
        output.push_str("    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {\n");

        // Options are already sorted by text length (descending) in analyzer.rs
        // Try each variant in order (longest first)
        for (idx, (option, variant_rust_name)) in options.iter().enumerate() {
            if idx > 0 {
                output.push_str("        let saved_pos = stream.position();\n");
            }

            output.push_str(&format!("        // Try {}\n", variant_rust_name));
            output.push_str("        {\n");

            match option {
                AnalyzedModifier::Atom((ident, _)) => {
                    output.push_str(&format!(
                        "            let saved_pos = stream.position();\n            if stream.expect_string(\"{}\").is_ok() {{\n",
                        ident
                    ));
                    output.push_str(&format!("                return Ok({}::{});\n", enum_name, variant_rust_name));
                    output.push_str("            }\n");
                    output.push_str("            stream.set_position(saved_pos);\n");
                }
                AnalyzedModifier::Sequence(items) => {
                    let mut sequence_code = String::new();
                    let mut tuple_vars = Vec::new();

                    for (item, item_rust_name) in items {
                        sequence_code.push_str(&self.generate_modifier_parser(item, item_rust_name));
                        tuple_vars.push(item_rust_name.clone());
                    }

                    let tuple_pattern = format!("({})", tuple_vars.join(", "));
                    let variant_args = tuple_vars.join(", ");
                    
                    output.push_str("            let saved_seq_pos = stream.position();\n");
                    output.push_str("            match (|| -> Result<_, PtxParseError> {\n");
                    output.push_str(&sequence_code);
                    output.push_str(&format!("                Ok({})\n", tuple_pattern));
                    output.push_str("            })() {\n");
                    output.push_str(&format!("                Ok({}) => {{\n", tuple_pattern));
                    output.push_str(&format!("                    return Ok({}::{}({}));\n", enum_name, variant_rust_name, variant_args));
                    output.push_str("                }\n");
                    output.push_str("                Err(_) => {\n");
                    output.push_str("                    stream.set_position(saved_seq_pos);\n");
                    output.push_str("                }\n");
                    output.push_str("            }\n");
                }
                _ => {
                    // For other complex types, try to parse them
                    output.push_str("            // TODO: implement parser for complex variant\n");
                }
            }

            output.push_str("        }\n");

            if idx > 0 {
                output.push_str("        stream.set_position(saved_pos);\n");
            }
        }

        // Generate error for when no variant matches
        output.push_str("        let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(Span { start: 0, end: 0 });\n");
        output.push_str(&format!("        let expected = &[{}];\n", 
            options.iter()
                .map(|(opt, _)| match opt {
                    AnalyzedModifier::Atom((ident, _)) => format!("\"{}\"", ident),
                    _ => "\"<complex>\"".to_string()
                })
                .collect::<Vec<_>>()
                .join(", ")
        ));
        output.push_str("        let found = stream.peek().map(|(t, _)| format!(\"{:?}\", t)).unwrap_or_else(|_| \"<end of input>\".to_string());\n");
        output.push_str("        Err(crate::parser::unexpected_value(span, expected, found))\n");
        output.push_str("    }\n");
        output.push_str("}\n");

        output
    }

}

// ============================================================================
// Module generation
// ============================================================================

/// Generate the content for parser mod.rs file
/// Generate the parser mod.rs file content
/// Takes a list of (module_name, opcodes, structs) tuples
/// Note: opcodes is now Vec<String> to support files with multiple opcodes
pub fn generate_parser_mod_rs_content(
    modules: &[(String, Vec<String>, Vec<(String, String)>)]
) -> String {
    let mut output = String::new();

    // Generate module declarations
    output.push_str("// Auto-generated module declarations\n");
    output.push_str("// DO NOT EDIT MANUALLY\n");
    output.push_str("#![allow(unused)]\n\n");

    output.push_str("use crate::parser::{PtxParser, PtxParseError, PtxTokenStream, Span};\n");
    output.push_str("use crate::r#type::instruction::{Instruction, InstructionWithPredicate, Predicate};\n");
    output.push_str("use crate::r#type::common::Operand;\n");
    output.push_str("use crate::lexer::PtxToken;\n\n");

    for (module_name, _, _) in modules {
        output.push_str(&format!("pub mod {};\n", module_name));
    }
    output.push_str("\n");

    // Generate the internal instruction parser (without label/pred)
    output.push_str("/// Parse instruction without label or predicate\n");
    output.push_str("fn parse_instruction_inner(stream: &mut PtxTokenStream) -> Result<Instruction, PtxParseError> {\n");
    output.push_str("    let start_pos = stream.position();\n");
    output.push_str("    \n");
    output.push_str("    // Peek at the opcode to determine which parser to try\n");
    output.push_str("    let opcode = if let Ok((PtxToken::Identifier(name), _)) = stream.peek() {\n");
    output.push_str("        name.as_str()\n");
    output.push_str("    } else {\n");
    output.push_str("        let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);\n");
    output.push_str("        return Err(crate::parser::unexpected_value(span, &[\"instruction opcode\"], \"not an identifier\"));\n");
    output.push_str("    };\n");
    output.push_str("    \n");
    output.push_str("    // Dispatch based on opcode\n");
    output.push_str("    match opcode {\n");

    // Group instructions by opcode
    // Now handles files with multiple opcodes (e.g., bar.txt has both "bar" and "barrier")
    use std::collections::BTreeMap;
    let mut opcode_map: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    for (module_name, opcodes, structs) in modules {
        for opcode in opcodes {
            for (section_name, struct_name) in structs {
                opcode_map.entry(opcode.clone())
                    .or_insert_with(Vec::new)
                    .push((module_name.clone(), section_name.clone(), struct_name.clone()));
            }
        }
    }

    // Generate match arms for each opcode
    for (opcode, parsers) in opcode_map {
        output.push_str(&format!("        \"{}\" => {{\n", opcode));
        for (module_name, section_name, struct_name) in parsers {
            output.push_str("            stream.set_position(start_pos);\n");
            // Use match instead of if-let to avoid retaining Result on stack
            output.push_str(&format!(
                "            match <crate::r#type::instruction::{}::{}::{} as PtxParser>::parse(stream) {{\n",
                module_name, section_name, struct_name
            ));
            output.push_str(&format!(
                "                Ok(inst) => return Ok(Instruction::{}(inst)),\n",
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
    output.push_str("    let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);\n");
    output.push_str("    Err(crate::parser::unexpected_value(span, &[\"valid PTX instruction\"], \"no matching instruction format\"))\n");
    output.push_str("}\n");
    output.push_str("\n");

    // Generate the main Instruction parser with label and predicate support
    output.push_str("impl PtxParser for InstructionWithPredicate {\n");
    output.push_str("    /// Parse a PTX instruction with optional label and predicate\n");
    output.push_str("    ///\n");
    output.push_str("    /// Format: [label:] [@{!}pred] instruction\n");
    output.push_str("    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {\n");
    output.push_str("        // Optional label (ends with colon)\n");
    output.push_str("        // Labels are handled by the module parser, not here\n");
    output.push_str("        \n");
    output.push_str("        // Optional predicate: @{!}pred or @!pred\n");
    output.push_str("        let predicate = if stream.check(|t| matches!(t, PtxToken::At)) {\n");
    output.push_str("            stream.consume()?; // consume @\n");
    output.push_str("            \n");
    output.push_str("            // Optional negation\n");
    output.push_str("            let negated = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();\n");
    output.push_str("\n");
    output.push_str("            // Predicate operand (can be register %p1 or identifier p)\n");
    output.push_str("            let operand = Operand::parse(stream)?;\n");
    output.push_str("\n");
    output.push_str("            Some(Predicate { negated, operand })\n");
    output.push_str("        } else {\n");
    output.push_str("            None\n");
    output.push_str("        };\n");
    output.push_str("        \n");
    output.push_str("        // Parse the actual instruction\n");
    output.push_str("        let instruction = parse_instruction_inner(stream)?;\n");
    output.push_str("        \n");
    output.push_str("        Ok(InstructionWithPredicate { predicate, instruction })\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    output.push_str("\n");
    output.push_str("// Backwards compatibility: Instruction can still be parsed directly\n");
    output.push_str("impl PtxParser for Instruction {\n");
    output.push_str("    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {\n");
    output.push_str("        Ok(InstructionWithPredicate::parse(stream)?.instruction)\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    output
}
