use std::collections::BTreeMap;

use crate::analyzer::{
    AnalyzedInstruction, AnalyzedModifier, AnalyzedOperandElement, AnalyzedSection,
};
use crate::naming;
use crate::r#type::OperatorToken;

/// Output collected when generating types for a PTX section
pub struct GeneratedTypeOutput {
    /// All generated Rust code for the section (enums + instruction structs)
    pub code: String,
    /// Struct names emitted for each instruction in order of generation
    pub instruction_structs: Vec<String>,
    /// Module name for this section
    pub module_name: String,
}

/// Represents an enum definition to be generated
struct EnumDefinition {
    variants: Vec<(String, String, Option<Vec<String>>)>, // (variant_name, raw_value, optional_tuple_types)
}

impl EnumDefinition {
    fn new() -> Self {
        Self { variants: Vec::new() }
    }

    fn add_variant(&mut self, variant: (String, String, Option<Vec<String>>)) {
        let (ref name, ref raw, ref tuple_types) = variant;
        if !self
            .variants
            .iter()
            .any(|existing| existing.0 == *name && existing.1 == *raw && existing.2 == *tuple_types)
        {
            self.variants.push(variant);
        }
    }
}

/// Generates Rust type definitions from analyzed PTX specifications
pub struct TypeGenerator {
    /// Pending enum definitions to emit
    pending_enums: BTreeMap<String, EnumDefinition>,
    enum_order: Vec<String>,
}

impl TypeGenerator {
    pub fn new() -> Self {
        Self {
            pending_enums: BTreeMap::new(),
            enum_order: Vec::new(),
        }
    }

    /// Generate Rust type definitions for an analyzed section
    pub fn generate(
        &mut self,
        section: &AnalyzedSection,
        section_idx: usize,
    ) -> GeneratedTypeOutput {
        let mut struct_output = String::new();
        let mut struct_names = Vec::new();

        // Generate type definitions for each instruction
        for instr in &section.0 {
            let struct_name = instr.rust_name.clone();
            let type_def = self.generate_instruction_type(instr);
            struct_names.push(struct_name);
            struct_output.push_str(&type_def);
            struct_output.push_str("\n\n");
        }

        // Emit all collected enum definitions for this section
        let mut enum_output = String::new();
        for enum_name in &self.enum_order {
            if let Some(enum_def) = self.pending_enums.get(enum_name) {
                enum_output.push_str(&Self::emit_enum_definition(enum_name, enum_def));
                enum_output.push_str("\n\n");
            }
        }

        // Clear both for the next section - each section needs its own enum definitions
        // because the same enum name can have different variants in different sections
        self.enum_order.clear();
        self.pending_enums.clear();

        // Generate module name from section index
        let module_name = format!("section_{}", section_idx);
        
        // Wrap everything in a module
        let mut output = String::new();
        output.push_str(&format!("pub mod {} {{\n", module_name));
        output.push_str("    use crate::r#type::common::*;\n\n");
        
        // Indent enum definitions
        for line in enum_output.lines() {
            if !line.is_empty() {
                output.push_str("    ");
            }
            output.push_str(line);
            output.push_str("\n");
        }
        
        // Indent struct definitions
        for line in struct_output.lines() {
            if !line.is_empty() {
                output.push_str("    ");
            }
            output.push_str(line);
            output.push_str("\n");
        }
        
        output.push_str("}\n");
        
        GeneratedTypeOutput {
            code: output,
            instruction_structs: struct_names,
            module_name,
        }
    }

    fn emit_enum_definition(enum_name: &str, enum_def: &EnumDefinition) -> String {
        let mut output = String::new();
        output.push_str(&format!("#[derive(Debug, Clone, PartialEq)]\n"));
        output.push_str(&format!("pub enum {} {{\n", enum_name));
        for (variant_name, raw_value, tuple_types) in &enum_def.variants {
            if let Some(types) = tuple_types {
                // Tuple variant: Variant(Type1, Type2, ...)
                output.push_str(&format!(
                    "    {}({}), // {}\n",
                    variant_name,
                    types.join(", "),
                    raw_value
                ));
            } else {
                // Simple variant: Variant
                output.push_str(&format!("    {}, // {}\n", variant_name, raw_value));
            }
        }
        output.push_str("}");
        output
    }

    fn get_or_create_enum(&mut self, enum_name: &str) -> &mut EnumDefinition {
        if !self.pending_enums.contains_key(enum_name) {
            self.enum_order.push(enum_name.to_string());
            self.pending_enums
                .insert(enum_name.to_string(), EnumDefinition::new());
        }
        self.pending_enums
            .get_mut(enum_name)
            .expect("enum must exist after insertion")
    }

    fn add_enum_variants(
        &mut self,
        enum_name: &str,
        variants: Vec<(String, String, Option<Vec<String>>)>,
    ) {
        let entry = self.get_or_create_enum(enum_name);
        for variant in variants {
            entry.add_variant(variant);
        }
    }

    /// Generate a struct definition for an instruction
    fn generate_instruction_type(&mut self, instr: &AnalyzedInstruction) -> String {
        let struct_name = &instr.rust_name;

        let mut output = String::new();

        output.push_str(&format!("#[derive(Debug, Clone, PartialEq)]\n"));
        output.push_str(&format!("pub struct {} {{\n", struct_name));

        // Generate fields from modifiers
        for (modifier, rust_name) in &instr.head.modifiers {
            let field_type = self.infer_modifier_type(modifier);
            let raw_form = self.get_modifier_raw_form(modifier);

            output.push_str(&format!("    pub {}: {}, // {}\n", rust_name, field_type, raw_form));
        }

        // Generate fields from operands
        for operand in &instr.operands {
            let mut operand_raw = String::new();

            // If operand has an operator (like {-} or {!}), add it as an optional field
            if let Some((operator, rust_name)) = &operand.operator {
                let op_prefix = match operator {
                    OperatorToken::Negate => "{-}",
                    OperatorToken::LogicalNot => "{!}",
                };
                operand_raw.push_str(op_prefix);
                output.push_str(&format!(
                    "    pub {}: bool, // {} operator\n",
                    rust_name, op_prefix
                ));
            }

            // Add the operand element raw form
            operand_raw.push_str(&self.get_operand_raw_form(&operand.operand));

            // Handle PipeChoice and PipeOptionalChoice specially - they need two fields
            use AnalyzedOperandElement::*;
            match &operand.operand {
                PipeChoice(((_, first_name), (_, second_name))) => {
                    // d|p generates: pub d: Operand, pub p: Operand
                    output.push_str(&format!(
                        "    pub {}: Operand, // first operand of {}\n",
                        first_name, operand_raw
                    ));
                    output.push_str(&format!(
                        "    pub {}: Operand, // second operand of {}\n",
                        second_name, operand_raw
                    ));
                }
                PipeOptionalChoice(((_, first_name), (_, second_name))) => {
                    // d{|p} generates: pub d: Operand, pub p: Option<Operand>
                    output.push_str(&format!(
                        "    pub {}: Operand, // first operand of {}\n",
                        first_name, operand_raw
                    ));
                    output.push_str(&format!(
                        "    pub {}: Option<Operand>, // optional second operand of {}\n",
                        second_name, operand_raw
                    ));
                }
                _ => {
                    // All other operand types - single field
                    let operand_type = self.get_operand_type(&operand.operand);
                    let field_name = Self::get_operand_rust_name(&operand.operand);
                    output.push_str(&format!(
                        "    pub {}: {}, // {}\n",
                        field_name, operand_type, operand_raw
                    ));
                }
            }

            // If operand has a modifier, generate field for it
            if let Some((modifier, rust_name)) = &operand.modifier {
                let field_type = self.infer_modifier_type(modifier);
                let mod_raw = self.get_modifier_raw_form(modifier);
                output.push_str(&format!("    pub {}: {}, // {}\n", rust_name, field_type, mod_raw));
            }
        }

        output.push_str("}");

        output
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
            CurlyGroup(items) | SquareGroup(items) => {
                // For groups, use the first item's name
                items.first().map(|(_, rust_name)| rust_name.clone()).unwrap_or_else(|| "group".to_string())
            }
            ParamList(rust_name) => rust_name.clone(),
            Choice { base, .. } => {
                // base.1 contains the PascalCase type name (e.g., "CpSize")
                // Convert to snake_case for use as field name (e.g., "cp_size")
                naming::to_snake_case(&base.1)
            }
        }
    }

    /// Get the raw PTX form of a modifier
    fn get_modifier_raw_form(&self, modifier: &AnalyzedModifier) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => ident.clone(),
            AnalyzedModifier::Choice { base, .. } => base.0.clone(),
            AnalyzedModifier::Sequence(items) => {
                items
                    .iter()
                    .map(|(item, _)| self.get_modifier_raw_form(item))
                    .collect::<Vec<_>>()
                    .join("")
            }
            AnalyzedModifier::Optional(inner) => {
                format!("{{{}}}", self.get_modifier_raw_form(&inner.0))
            }
        }
    }

    /// Get the raw PTX form of an operand element
    fn get_operand_raw_form(&self, operand: &AnalyzedOperandElement) -> String {
        use AnalyzedOperandElement::*;
        match operand {
            Item((ident, _)) => ident.clone(),
            Address((ident, _)) => format!("[{}]", ident),
            Optional((ident, _)) => format!("{{, {}}}", ident),
            ParenthesizedOperand((ident, _)) => format!("({})", ident),
            PipeChoice(((first, _), (second, _))) => format!("{}|{}", first, second),
            PipeOptionalChoice(((first, _), (second, _))) => format!("{}{{|{}}}", first, second),
            CurlyGroup(operands) => {
                let parts: Vec<String> = operands
                    .iter()
                    .map(|(ident, _)| ident.clone())
                    .collect();
                format!("{{{}}}", parts.join(", "))
            }
            SquareGroup(operands) => {
                let parts: Vec<String> = operands
                    .iter()
                    .map(|(ident, _)| ident.clone())
                    .collect();
                format!("[{}]", parts.join(", "))
            }
            ParamList(_) => "(param-list)".to_string(),
            ImmediateNumber((num, _)) => num.clone(),
            Choice { base, .. } => base.0.clone(),
        }
    }

    /// Get the Rust type for an operand element
    fn get_operand_type(&mut self, operand: &AnalyzedOperandElement) -> String {
        use AnalyzedOperandElement::*;
        match operand {
            // Optional operands should be Option<Operand>
            Optional(_) => "Option<Operand>".to_string(),
            // Immediate numbers are just markers
            ImmediateNumber(_) => "()".to_string(),
            // Address operands use AddressOperand type
            Address(_) => "AddressOperand".to_string(),
            // Choice operands should generate an enum
            Choice { base, options } => self.generate_operand_choice_enum(&base.1, options),
            // ParamList is a list of operands
            ParamList(_) => "Vec<Operand>".to_string(),
            // SquareGroup should be a tuple
            SquareGroup(operands) => {
                let element_types: Vec<String> = vec!["Operand".to_string(); operands.len()];
                format!("({})", element_types.join(", "))
            }
            // CurlyGroup should be a tuple
            CurlyGroup(operands) => {
                let element_types: Vec<String> = vec!["Operand".to_string(); operands.len()];
                format!("({})", element_types.join(", "))
            }
            // PipeChoice needs two operand fields
            PipeChoice(_) => "Operand".to_string(),
            // PipeOptionalChoice needs operand + optional operand
            PipeOptionalChoice(_) => "Operand".to_string(),
            // All other operands
            _ => "Operand".to_string(),
        }
    }

    /// Generate an enum for operand choice (like cp-size = { 4, 8, 16 })
    fn generate_operand_choice_enum(
        &mut self,
        enum_name: &str,
        options: &[String],
    ) -> String {
        // Generate enum variants from options
        let mut variants = Vec::new();
        for option in options {
            // Sanitize the option to create a valid variant name
            let variant_name = self.sanitize_variant_name(option);
            variants.push((variant_name, option.clone(), None));
        }

        // Add to pending enums
        self.add_enum_variants(enum_name, variants);

        enum_name.to_string()
    }

    /// Generate an enum definition for a Choice modifier
    fn generate_choice_enum(&mut self, modifier: &AnalyzedModifier) -> String {
        // Extract enum name and options from modifier
        let (enum_name, options) = match modifier {
            AnalyzedModifier::Choice { base, options } => (&base.1, options),
            _ => panic!("Expected Choice modifier"),
        };

        // Generate enum variants from options
        let mut variants = Vec::new();
        for (option, variant_rust_name) in options {
            let raw_value = self.get_modifier_raw_form(option);

            // Check if this option is a Sequence - if so, generate tuple types
            let tuple_types = match option {
                AnalyzedModifier::Sequence(items) => {
                    let types: Vec<String> = items
                        .iter()
                        .map(|(item, _)| self.infer_modifier_type(item))
                        .collect();
                    Some(types)
                }
                _ => None,
            };

            variants.push((variant_rust_name.clone(), raw_value, tuple_types));
        }

        // Add to pending enums
        self.add_enum_variants(enum_name, variants);

        enum_name.clone()
    }

    /// Infer a Rust type from a modifier
    fn infer_modifier_type(&mut self, modifier: &AnalyzedModifier) -> String {
        match modifier {
            AnalyzedModifier::Atom(_) => "()".to_string(),
            AnalyzedModifier::Choice { .. } => {
                // Generate the enum and return its name
                self.generate_choice_enum(modifier)
            }
            AnalyzedModifier::Sequence(items) => {
                // Generate tuple type (Type1, Type2, ...)
                let types: Vec<String> = items
                    .iter()
                    .map(|(item, _)| self.infer_modifier_type(item))
                    .collect();
                format!("({})", types.join(", "))
            }
            AnalyzedModifier::Optional(inner) => {
                // For Optional(Atom), use bool instead of Option<()>
                match &inner.0 {
                    AnalyzedModifier::Atom(_) => "bool".to_string(),
                    _ => format!("Option<{}>", self.infer_modifier_type(&inner.0))
                }
            }
        }
    }

    fn sanitize_variant_name(&self, s: &str) -> String {
        let cleaned = s
            .trim_start_matches('.')
            .trim_start_matches("::")
            .replace("::", "_")
            .replace('.', "_")
            .replace('-', "_")
            .replace('*', "")
            .replace('+', "plus");

        let pascal = cleaned
            .split('_')
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<String>();

        if pascal.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            format!("_{}", pascal)
        } else {
            pascal
        }
    }
}

// ============================================================================
// Module generation
// ============================================================================

/// Generate the content for mod.rs file in the type module
/// Generate the mod.rs file content with proper module structure
/// Takes module info with format: (filename, Vec<(section_name, struct_name)>)
pub fn generate_mod_rs_content_v2(
    modules: &[(String, Vec<(String, String)>)]
) -> String {
    let mut output = String::new();

    // Generate module declarations
    output.push_str("// Auto-generated module declarations\n");
    output.push_str("// DO NOT EDIT MANUALLY\n");
    output.push_str("#![allow(unused)]\n\n");

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {};\n", module_name));
    }
    output.push_str("\n");

    // Generate Instruction enum
    output.push_str("/// Top-level instruction type encompassing all PTX instructions\n");
    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str("pub enum Instruction {\n");

    for (module_name, structs) in modules {
        for (section_name, struct_name) in structs {
            output.push_str(&format!(
                "    {}({}::{}::{}),\n",
                struct_name, module_name, section_name, struct_name
            ));
        }
    }

    output.push_str("}\n");

    output
}

pub fn generate_mod_rs_content(modules: &[GeneratedTypeOutput]) -> String {
    let mut output = String::new();

    // Generate module declarations
    output.push_str("// Auto-generated module declarations\n");
    output.push_str("// DO NOT EDIT MANUALLY\n\n");

    for module in modules {
        output.push_str(&format!("pub mod {};\n", module.module_name));
    }
    output.push_str("\n");

    // Generate Instruction enum
    output.push_str("/// Top-level instruction type encompassing all PTX instructions\n");
    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str("pub enum Instruction {\n");

    for module in modules {
        for struct_name in &module.instruction_structs {
            output.push_str(&format!(
                "    {}({}::{}),\n",
                struct_name, module.module_name, struct_name
            ));
        }
    }

    output.push_str("}\n");

    output
}
